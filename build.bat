@echo off
if exist "dist.zip" del "dist.zip"

cargo build --release

mkdir "dist\logs"
xcopy "target\release\vgi-il-sas-extractions.exe" "dist\" /f
xcopy "config" "dist\config" /s /e /i /d 

powershell Compress-Archive -Path "dist" -DestinationPath "dist.zip"
rmdir /S /Q "dist"
