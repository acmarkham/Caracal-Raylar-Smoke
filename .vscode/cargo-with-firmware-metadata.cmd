@echo off
powershell.exe -NoProfile -ExecutionPolicy Bypass -File "%~dp0..\scripts\cargo-with-firmware-metadata.ps1" %*
exit /b %ERRORLEVEL%
