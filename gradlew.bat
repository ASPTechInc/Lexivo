@echo off
set SCRIPT_DIR=%~dp0
call "%SCRIPT_DIR%android\gradlew.bat" -p "%SCRIPT_DIR%" %*
