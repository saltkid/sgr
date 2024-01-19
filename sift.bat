@echo off

if not exist "%~dp0dirs.txt" (
    echo %userprofile%\projects>"%~dp0dirs.txt"
)

setlocal enabledelayedexpansion
if not "%~1"=="" (
    if /i "%~1"=="add" (
        rem check if there's a path specified after "add"
        if not "%~2"=="" (
            rem check if the second argument is a valid path and contains at least one git repo
            if exist "%~2" (
                echo set count=0
                set "count=0"
                for /f "delims=" %%j in ('dir /b /ad /s "%~2\*.git" ^| sed "s/\\.git$//"') do (
                    set /a count+=1
                )
                if !count! gtr 0 (
                    rem Validate if %~f2 already exists in %~dp0dirs.txt
                    for /f "usebackq delims=" %%k in ("%~dp0dirs.txt") do (
                        if "%%k"=="%~f2" (
                            echo '%~f2' already exists in dirs.txt
                            exit /b 1
                        )
                    )
                    echo %~f2>>"%~dp0dirs.txt"
                    echo added "%~2"
                    exit /b 0

                ) else (
                    echo No Git repositories found under "%~2"
                    exit /b 1
                )

            ) else (
                echo "%~2" is not a valid path
                exit /b 1
            )

        ) else (
            echo No path specified for "sift add"
            exit /b 1
        )
    )
    if /i "%~1"=="list" (
        echo ---------------------------------------------------
        echo idx ^| dirs
        echo ---------------------------------------------------
        set "line_num=1"
        set "list_arg=%~2"

        if not "%~2"=="" (
            set /a "hyphen_position=0"
            set /a "hyphen_count=0"
            rem identify the position of the hyphen
            for /l %%i in (0,1,100) do (
                set "char=!list_arg:~%%i,1!"
                if "!char!"=="-" (
                    if %%i==0 (
                        echo negative numbers are not allowed
                        exit /b 1
                    )
                    set /a hyphen_count+=1
                    set /a hyphen_position=%%i
                )
            )

            if !hyphen_count!==1 (
                rem split %2 into two parts
                for /f "tokens=1,* delims=-" %%a in ("!list_arg!") do (
                    set "range_start=%%a"
                    set "range_end=%%b"
                )

                rem check if each part is a number
                if 1!range_start! neq +1!range_start! (
                    echo !range_start! is not a valid starting range
                    exit /b 1
                )
                if 1!range_end! neq +1!range_end! (
                    echo !range_end! is not a valid ending range
                    exit /b 1
                )
                rem check if starting range is smaller than ending range
                if !range_start! gtr !range_end! (
                    echo starting range '!range_start!' must be smaller than ending range '!range_end!'
                    exit /b 1
                )
                rem valid range, get dirs
                for /f "usebackq delims=" %%i in ("%~dp0dirs.txt") do (
                    if !line_num! geq !range_start! if !line_num! leq !range_end! (
                        set "pad_num=0!line_num!"
                        echo  !pad_num:~-2! ^| "%%i"
                    )
                    set /a line_num+=1
                )
                echo ---------------------------------------------------
                exit /b 0
            )

            if !hyphen_count!==0 (
                rem check if a number
                if 1%2 neq +1%2 (
                    echo %~2 is not a valid line number
                    exit /b 1
                )

                set "check_idx=%~2"
                for /f "usebackq delims=" %%i in ("%~dp0dirs.txt") do (
                    if !line_num!==!check_idx! (
                        set "pad_num=0!line_num!"
                        echo  !pad_num:~-2! ^| "%%i"
                        echo ---------------------------------------------------
                        exit /b 0
                    )
                    set /a line_num+=1
                )
                echo line %~2 not found
                echo ---------------------------------------------------
                exit /b 1
            )

        )

        for /f "usebackq delims=" %%i in ("%~dp0dirs.txt") do (
            if not "%%i"=="" (
                set "pad_num=0!line_num!"
                echo  !pad_num:~-2! ^| "%%i"
                set /a line_num+=1
            )
        )
        echo ---------------------------------------------------
        exit /b 0
    )
)
endlocal

rem select git repos from the list of directories in dirs.txt
for /f "usebackq delims=" %%i in (`type %~dp0dirs.txt`) do (
    for /f "delims=" %%j in ('dir /b /ad /s "%%i\*.git" ^| sed "s/\\.git$//"') do (
    	echo %%j>>"%~dp0temp_repos.txt"
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

if "%selected%"=="" (
    echo No directory selected.
    set "selected="
    del "%~dp0temp_repos.txt
    exit /b 0
)

cd %selected%
echo Now in: %selected%
rem delete selected environment variable
set "selected="
del %~dp0temp_repos.txt
