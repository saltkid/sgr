@echo off

rem to properly escape ansi color codes
for /F "tokens=1,2 delims=#" %%a in ('"prompt #$H#$E# & echo on & for %%b in (1) do rem"') do set ESC=%%b

rem create dirs.txt if doesn't exist yet
if not exist "%~dp0dirs.txt" (
    echo %userprofile%\projects>"%~dp0dirs.txt"
)

rem properly pass args to sgr
if "%~1"=="" (
    goto select_repos
)
if not "%~1"=="" (
    sugar %*
    goto end
)

:select_repos
    rem select the entire message sent by sgr
    for /f "tokens=*" %%i in ('sugar') do (
        set "selected_path=%%i"
    )

    rem select only the header (should be either [WARN] or a path)
    for /f "tokens=1 delims= " %%a in ("%selected_path%") do (
        if "%%a"=="%ESC%[31m[ERROR]%ESC%[0m" (
            rem echo error message
            echo %selected_path%
            goto end
        )
        if "%%a"=="%ESC%[33m[WARN]%ESC%[0m" (
            rem echo warning message
            echo %selected_path%
            goto end
        )
    )

    cd /d "%selected_path%" 
    echo Changed Directory to "%selected_path%"

rem remove env vars
:end
    set "selected_path="
    set "ESC="
