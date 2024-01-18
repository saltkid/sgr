@echo off

if not exist "%~dp0dirs.txt" (
    echo %userprofile%\projects>"%~dp0dirs.txt"
)


if "%~1" neq "" (
    set "selected=%~1"
) else (
    rem select git repos from the list of directories in dirs.txt
    for /f "usebackq delims=" %%i in (`type %~dp0dirs.txt`) do (
        for /f "delims=" %%j in ('dir /b /ad /s "%%i\*.git" ^| sed "s/\\.git$//"') do (
		echo %%j>>%~dp0temp_repos.txt
        )
    )
 
    rem check if temp_repos.txt exists aka if found any git repos
    if not exist "%~dp0temp_repos.txt" (
        echo No local git repos found under:
        type "%~dp0dirs.txt"
        set "selected="
        exit /b 0
    )

    for /f "delims=" %%i in ('type %~dp0temp_repos.txt ^| fzf') do set "selected=%%~i"
)

if "%selected%"=="" (
    echo No directory selected.
    set "selected="
    del %~dp0temp_repos.txt
    exit /b 0
)

cd %selected%
echo Now in: %selected%
rem delete selected environment variable
set "selected="
del %~dp0temp_repos.txt
