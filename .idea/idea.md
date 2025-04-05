# yt-dlp_bypass

## 目的

指定ディレクトリにある`yt-dlp.exe`(A)の代わりにバイナリを配置します。
外部アプリから配置された`yt-dlp.exe`(A)へ実行されたオプションをすべて同ディレクトリに配置した`yt-dlp_org.exe`(B)にバイパスします。
その際オプション`--cookies cookies.txt`を追加します

- `yt-dlp.exe`(A): 配置先のバイナリ(本アプリケーション)
- `yt-dlp_org.exe`(B): 別場所のオリジナルyt-dlp
