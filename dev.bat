@echo off
@REM %~dp0
@REM https://stackoverflow.com/a/17064031/14791867
@REM echo %~dp0  
set PATH=%~dp0;%PATH%  
echo %PATH%  
cd %~dp0
code .