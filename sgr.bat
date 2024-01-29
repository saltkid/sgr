@echo off

if not exist "%~dp0dirs.txt" (
    echo %userprofile%\projects>"%~dp0dirs.txt"
)

if "%~1"=="" (
    goto select_repos
) else (
    cargo run %~1 %~2
    goto end
)

:select_repos
for /f %%i in ('cargo run') do (
    set "selected_path=%%i"
)
if "%selected_path%"=="" (
    goto end
)

cd /d "%selected_path%" 
echo Changed Directory to "%selected_path%"

:end
set "selected_path="
