#!/bin/bash

cross build --target x86_64-pc-windows-gnu
cp target/x86_64-pc-windows-gnu/debug/yt-dlp_bypass.exe target/x86_64-pc-windows-gnu/debug/yt-dlp.exe
