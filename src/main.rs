use std::process::Command;
use std::env;
use yt_dlp_bypass::{get_original_yt_dlp_path, get_proxy_url};

fn main() {
    // オリジナルのyt-dlpのパスを取得
    let original_yt_dlp = get_original_yt_dlp_path();
    eprintln!("original_yt_dlp: {}", original_yt_dlp.display());
    
    // コマンドライン引数を取得（最初の引数は自己実行ファイル名なのでスキップ）
    let mut cmd_args: Vec<String> = env::args().skip(1).collect();
    
    // プロキシ設定がある場合は追加
    if let Some(proxy_url) = get_proxy_url() {
        cmd_args.push("--proxy".to_string());
        cmd_args.push(proxy_url);
    }

    eprintln!("cmd_args: {:?}", cmd_args);
    
    // オリジナルのyt-dlpを実行
    let output = Command::new(original_yt_dlp)
        .args(&cmd_args)
        .output()
        .expect("yt-dlpの実行に失敗しました");
    
    // 出力を表示
    println!("{}", String::from_utf8_lossy(&output.stdout));
    eprintln!("{}", String::from_utf8_lossy(&output.stderr));
}
