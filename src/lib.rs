use std::path::PathBuf;
use std::env;

pub fn get_original_yt_dlp_path() -> PathBuf {
    // 環境変数からオリジナルのyt-dlpのパスを取得
    if let Ok(path) = env::var("ORIGINAL_YT_DLP_PATH") {
        return PathBuf::from(path);
    }
    
    // デフォルトのパスを返す（カレントディレクトリのyt-dlp_org.exe）
    PathBuf::from("yt-dlp_org.exe")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use tempfile::tempdir;

    #[test]
    fn test_get_original_yt_dlp_path_with_env() {
        // 一時ディレクトリを作成
        let temp_dir = tempdir().unwrap();
        let test_path = temp_dir.path().join("yt-dlp.exe");
        
        // 環境変数を設定
        env::set_var("ORIGINAL_YT_DLP_PATH", test_path.to_str().unwrap());
        
        // パスを取得して検証
        let path = get_original_yt_dlp_path();
        assert_eq!(path, test_path);
        
        // 環境変数をクリア
        env::remove_var("ORIGINAL_YT_DLP_PATH");
    }

    #[test]
    fn test_get_original_yt_dlp_path_default() {
        // 環境変数が設定されていない場合のデフォルトパスを検証
        let path = get_original_yt_dlp_path();
        assert_eq!(path, PathBuf::from("yt-dlp_org.exe"));
    }
} 