use std::process::Command;
use std::env;
use yt_dlp_bypass::get_original_yt_dlp_path;


fn main() {
    // オリジナルのyt-dlpのパスを取得
    let original_yt_dlp = get_original_yt_dlp_path();
    
    // 現在の実行ファイルのパスを取得
    let current_exe = env::current_exe().expect("実行ファイルのパスを取得できませんでした");
    
    // コマンドライン引数を取得（最初の引数は自己実行ファイル名なのでスキップ）
    let mut cmd_args: Vec<String> = env::args().skip(1).collect();
    
    // cookies.txtのパスを追加
    let cookies_path = current_exe.parent().unwrap().join("cookies.txt");
    cmd_args.push("--cookies".to_string());
    cmd_args.push(cookies_path.to_str().unwrap().to_string());
    
    // オリジナルのyt-dlpを実行
    let output = Command::new(original_yt_dlp)
        .args(&cmd_args)
        .output()
        .expect("yt-dlpの実行に失敗しました");
    
    // 出力を表示
    println!("{}", String::from_utf8_lossy(&output.stdout));
    eprintln!("{}", String::from_utf8_lossy(&output.stderr));
}
