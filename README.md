# yt-dlp_bypass_rs

## 目的
YoutubeのBot対策により動画が再生されない問題を一時解決するためのツール  
yt-dlp.exeに対するコマンドをバイパスしてオプションを追加する  

## 結論
そもそもyt-dlp+Cookies.txtで動かなかっ.tar

## ビルド

```bash
# incontainer
cross build --target x86_64-pc-windows-gnu
# cp target/x86_64-pc-windows-gnu/debug/yt-dlp_bypass.exe path/to/yt-dlp.exe
```