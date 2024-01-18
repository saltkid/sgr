@echo off

if "%~1" neq "" (
    set "selected=%~1"
) else (
    rem select git repos
    for /f "delims=" %%i in ('dir /b /ad /s "%USERPROFILE%\projects\*" ^| findstr /i "\\\.git$" ^| sed "s/\\.git$//" ^| fzf') do set "selected=%%~i"
)

if "%selected%"=="" (
    exit /b 0
)

cd %selected%
rem delete selected environment variable
set "selected="
