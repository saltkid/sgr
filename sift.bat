@echo off

if "%~1" neq "" (
    set "selected=%~1"
) else (
    rem select git repos from the list of directories in dirs.txt
    for /f "usebackq delims=" %%i in (`type %~dp0dirs.txt`) do (
        for /f "delims=" %%j in ('dir /b /ad /s "%%i\\*.git" ^| sed "s/\\.git$//"') do (
		echo %%j>>%~dp0temp_repos.txt
        )
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
