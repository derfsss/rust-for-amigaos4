@echo off
setlocal enabledelayedexpansion
REM cargo-amiga.bat — Project management wrapper for Rust-for-AmigaOS4
REM
REM Usage:
REM   cargo-amiga.bat new <name> [--mode app|driver]
REM   cargo-amiga.bat build [<project-path>]
REM   cargo-amiga.bat clean [<project-path>]
REM   cargo-amiga.bat run [<project-path>] [--target NAME] [--args "..."]
REM   cargo-amiga.bat test [<project-path>] --wait REGEX [--target NAME] [--args "..."] [--timeout N]
REM   cargo-amiga.bat setup
REM   cargo-amiga.bat help

set "REPO_ROOT=%~dp0"
if "%REPO_ROOT:~-1%"=="\" set "REPO_ROOT=%REPO_ROOT:~0,-1%"

if "%1"=="" goto :cmd_help
if "%1"=="new" goto :cmd_new
if "%1"=="build" goto :cmd_build
if "%1"=="clean" goto :cmd_clean
if "%1"=="run" goto :cmd_run
if "%1"=="test" goto :cmd_test
if "%1"=="setup" goto :cmd_setup
if "%1"=="help" goto :cmd_help
if "%1"=="-h" goto :cmd_help
if "%1"=="--help" goto :cmd_help

echo ERROR: Unknown subcommand: %1. Run 'cargo-amiga.bat help' for usage.
exit /b 1

REM =========================================================================
REM new — scaffold a project from a template
REM =========================================================================
:cmd_new
set "NAME="
set "MODE=app"
shift

:parse_new_args
if "%1"=="" goto :do_new
if "%1"=="--mode" goto :parse_mode
set "ARG=%1"
if "!ARG:~0,1!"=="-" (
    echo ERROR: Unknown option: %1
    exit /b 1
)
if not defined NAME (
    set "NAME=%1"
    shift
    goto :parse_new_args
)
echo ERROR: Unexpected argument: %1
exit /b 1

:parse_mode
shift
if "%1"=="" (
    echo ERROR: --mode requires a value ^(app or driver^).
    exit /b 1
)
set "MODE=%1"
shift
goto :parse_new_args

:do_new
if not defined NAME (
    echo ERROR: Usage: cargo-amiga.bat new ^<name^> [--mode app^|driver]
    exit /b 1
)

REM Validate mode
if not "%MODE%"=="app" if not "%MODE%"=="driver" (
    echo ERROR: Unknown mode '%MODE%'. Use 'app' or 'driver'.
    exit /b 1
)

REM Validate name — basic check (non-empty, no spaces)
echo %NAME%| findstr /r "[ ]" >nul 2>&1 && (
    echo ERROR: Project name must not contain spaces.
    exit /b 1
)

set "TEMPLATE_DIR=%REPO_ROOT%\templates\%MODE%"
set "TARGET_DIR=%REPO_ROOT%\%NAME%"

if not exist "%TEMPLATE_DIR%\" (
    echo ERROR: Template directory not found: templates\%MODE%
    exit /b 1
)

if exist "%TARGET_DIR%" (
    echo ERROR: Directory already exists: %NAME%
    exit /b 1
)

REM Compute BIN_NAME (underscores -> hyphens) and LIB_NAME (hyphens -> underscores)
set "BIN_NAME=%NAME:_=-%"
set "LIB_NAME=%NAME:-=_%"

echo Creating %MODE% project '%NAME%'...

REM Copy template tree (/H includes hidden files like .cargo)
xcopy /E /I /Q /H "%TEMPLATE_DIR%" "%TARGET_DIR%" >nul

REM --- Cargo.toml and Makefile replacements ---
if "%MODE%"=="app" goto :replace_app
goto :replace_driver

:replace_app
REM App template: name = "myapp" appears twice — replace all, then fix [lib] name to use underscores
powershell -Command "(Get-Content '%TARGET_DIR%\Cargo.toml') -replace 'name = \"myapp\"', 'name = \"%BIN_NAME%\"' | Set-Content '%TARGET_DIR%\Cargo.toml'"
powershell -Command "$lines = Get-Content '%TARGET_DIR%\Cargo.toml'; $inLib = $false; for ($i = 0; $i -lt $lines.Count; $i++) { if ($lines[$i] -match '^\[lib\]') { $inLib = $true; continue } if ($inLib -and $lines[$i] -match '^name\s*=') { $lines[$i] = 'name = \"%LIB_NAME%\"'; break } if ($lines[$i] -match '^\[') { $inLib = $false } } $lines | Set-Content '%TARGET_DIR%\Cargo.toml'"
powershell -Command "(Get-Content '%TARGET_DIR%\Makefile') -replace 'TARGET    = myapp', 'TARGET    = %BIN_NAME%' -replace 'release/libmyapp\.a', 'release/lib%LIB_NAME%.a' | Set-Content '%TARGET_DIR%\Makefile'"
goto :replace_done

:replace_driver
REM Driver template: name = "my-handler" (package), name = "my_handler" (lib)
powershell -Command "(Get-Content '%TARGET_DIR%\Cargo.toml') -replace 'name = \"my-handler\"', 'name = \"%BIN_NAME%\"' -replace 'name = \"my_handler\"', 'name = \"%LIB_NAME%\"' | Set-Content '%TARGET_DIR%\Cargo.toml'"
powershell -Command "(Get-Content '%TARGET_DIR%\Makefile') -replace 'TARGET    = my-handler', 'TARGET    = %BIN_NAME%' -replace 'release/libmy_handler\.a', 'release/lib%LIB_NAME%.a' | Set-Content '%TARGET_DIR%\Makefile'"
goto :replace_done

:replace_done

echo.
echo Project '%NAME%' created (%MODE% mode).
echo.
echo Next steps:
echo   cargo-amiga.bat build %NAME%
echo   :: or:
echo   build.bat %NAME%
exit /b 0

REM =========================================================================
REM build — compile a project
REM =========================================================================
:cmd_build
if not "%2"=="" (
    call "%REPO_ROOT%\build.bat" "%2"
    exit /b !errorlevel!
)

REM No path given — use current directory relative to repo root
set "CWD=%CD%"
REM Check if CWD is under REPO_ROOT
echo %CWD% | findstr /i /b "%REPO_ROOT%" >nul 2>&1
if errorlevel 1 (
    echo ERROR: Current directory is not inside the repository.
    exit /b 1
)
REM Strip repo root prefix to get relative path
call set "PROJECT=%%CWD:%REPO_ROOT%\=%%"
call "%REPO_ROOT%\build.bat" "%PROJECT%"
exit /b !errorlevel!

REM =========================================================================
REM clean — clean a project
REM =========================================================================
:cmd_clean
if not "%2"=="" (
    call "%REPO_ROOT%\build.bat" "%2" clean
    exit /b !errorlevel!
)

REM No path given — use current directory relative to repo root
set "CWD=%CD%"
echo %CWD% | findstr /i /b "%REPO_ROOT%" >nul 2>&1
if errorlevel 1 (
    echo ERROR: Current directory is not inside the repository.
    exit /b 1
)
call set "PROJECT=%%CWD:%REPO_ROOT%\=%%"
call "%REPO_ROOT%\build.bat" "%PROJECT%" clean
exit /b !errorlevel!

REM =========================================================================
REM run / test — build, then deploy + launch on a fleet target (QEMU or
REM hardware) via the amiga-fleet CLI (MCP-AmigaOS4). 'run' just launches;
REM 'test' also waits for a serial marker and exits non-zero if it is not
REM seen (a headless smoke). Mirrors cmd_run/cmd_test in cargo-amiga.sh.
REM =========================================================================
:cmd_run
shift
set "PROJECT="
set "TGT="
set "RUN_ARGS="

:parse_run_args
if "%~1"=="" goto :do_run
if "%~1"=="--target" (
    set "TGT=%~2"
    shift & shift
    goto :parse_run_args
)
if "%~1"=="--args" (
    set "RUN_ARGS=%~2"
    shift & shift
    goto :parse_run_args
)
set "ARG=%~1"
if "!ARG:~0,1!"=="-" (
    echo ERROR: run: unknown flag %~1
    exit /b 1
)
if not defined PROJECT (
    set "PROJECT=%~1"
    shift
    goto :parse_run_args
)
echo ERROR: Unexpected argument: %~1
exit /b 1

:do_run
call :resolve_project || exit /b 1
call :find_fleet || exit /b 1
call "%REPO_ROOT%\build.bat" "%PROJECT%" || exit /b 1
set "EXTRA="
if defined TGT set "EXTRA=!EXTRA! --target "!TGT!""
if defined RUN_ARGS set "EXTRA=!EXTRA! --args="!RUN_ARGS!""
pushd "%FLEET_HOST%"
set "PYTHONPATH=src"
"%FLEET_PY%" -m amiga_fleet_mcp.cli qemu_run --exe "%EXE%" --start --stop!EXTRA!
set "RC=!errorlevel!"
popd
exit /b !RC!

:cmd_test
shift
set "PROJECT="
set "TGT="
set "RUN_ARGS="
set "WAIT="
set "TIMEOUT=240"

:parse_test_args
if "%~1"=="" goto :do_test
if "%~1"=="--target" (
    set "TGT=%~2"
    shift & shift
    goto :parse_test_args
)
if "%~1"=="--args" (
    set "RUN_ARGS=%~2"
    shift & shift
    goto :parse_test_args
)
if "%~1"=="--wait" (
    set "WAIT=%~2"
    shift & shift
    goto :parse_test_args
)
if "%~1"=="--timeout" (
    set "TIMEOUT=%~2"
    shift & shift
    goto :parse_test_args
)
set "ARG=%~1"
if "!ARG:~0,1!"=="-" (
    echo ERROR: test: unknown flag %~1
    exit /b 1
)
if not defined PROJECT (
    set "PROJECT=%~1"
    shift
    goto :parse_test_args
)
echo ERROR: Unexpected argument: %~1
exit /b 1

:do_test
if not defined WAIT (
    echo ERROR: test: --wait REGEX is required ^(the serial marker that means success^).
    exit /b 1
)
call :resolve_project || exit /b 1
call :find_fleet || exit /b 1
call "%REPO_ROOT%\build.bat" "%PROJECT%" || exit /b 1
set "EXTRA="
if defined TGT set "EXTRA=!EXTRA! --target "!TGT!""
if defined RUN_ARGS set "EXTRA=!EXTRA! --args="!RUN_ARGS!""
pushd "%FLEET_HOST%"
set "PYTHONPATH=src"
"%FLEET_PY%" -m amiga_fleet_mcp.cli qemu_run --exe "%EXE%" --wait "%WAIT%" --timeout %TIMEOUT% --start --stop!EXTRA!
set "RC=!errorlevel!"
popd
exit /b !RC!

REM -------------------------------------------------------------------------
REM resolve_project — fill PROJECT (if empty, from CWD relative to repo root),
REM PROJ_DIR, and EXE (the Makefile's TARGET inside PROJ_DIR).
REM -------------------------------------------------------------------------
:resolve_project
if defined PROJECT goto :resolve_project_dir
set "CWD=%CD%"
echo %CWD% | findstr /i /b "%REPO_ROOT%" >nul 2>&1
if errorlevel 1 (
    echo ERROR: Current directory is not inside the repository ^(pass a project path^).
    exit /b 1
)
call set "PROJECT=%%CWD:%REPO_ROOT%\=%%"

:resolve_project_dir
if "%PROJECT:~1,1%"==":" (
    set "PROJ_DIR=%PROJECT%"
) else (
    set "PROJ_DIR=%REPO_ROOT%\%PROJECT%"
)
if not exist "%PROJ_DIR%\Makefile" (
    echo ERROR: no Makefile in %PROJ_DIR%
    exit /b 1
)
set "TARGET_NAME="
for /f "usebackq delims=" %%T in (`powershell -NoProfile -Command "(Select-String -Path '%PROJ_DIR%\Makefile' -Pattern '^TARGET\s*=\s*(\S+)' | Select-Object -First 1).Matches[0].Groups[1].Value"`) do set "TARGET_NAME=%%T"
if not defined TARGET_NAME (
    echo ERROR: could not read TARGET from %PROJ_DIR%\Makefile
    exit /b 1
)
set "EXE=%PROJ_DIR%\%TARGET_NAME%"
exit /b 0

REM -------------------------------------------------------------------------
REM find_fleet — locate the MCP-AmigaOS4 host dir (which has the fleet CLI)
REM and a python to run it with. Override with AMIGA_FLEET_MCP_HOST; else
REM look for a sibling checkout. Sets FLEET_HOST and FLEET_PY.
REM -------------------------------------------------------------------------
:find_fleet
set "FLEET_HOST="
if defined AMIGA_FLEET_MCP_HOST (
    set "FLEET_HOST=%AMIGA_FLEET_MCP_HOST%"
    goto :find_fleet_py
)
if exist "%REPO_ROOT%\..\..\MCP-AmigaOS4\host\src\amiga_fleet_mcp\" (
    set "FLEET_HOST=%REPO_ROOT%\..\..\MCP-AmigaOS4\host"
    goto :find_fleet_py
)
if exist "%REPO_ROOT%\..\MCP-AmigaOS4\host\src\amiga_fleet_mcp\" (
    set "FLEET_HOST=%REPO_ROOT%\..\MCP-AmigaOS4\host"
    goto :find_fleet_py
)
echo ERROR: amiga-fleet CLI not found. Set AMIGA_FLEET_MCP_HOST=path\to\MCP-AmigaOS4\host or place MCP-AmigaOS4 as a sibling of this repo.
exit /b 1

:find_fleet_py
set "FLEET_PY=%FLEET_HOST%\.venv\Scripts\python.exe"
if exist "%FLEET_PY%" exit /b 0
set "FLEET_PY=python"
exit /b 0

REM =========================================================================
REM setup — one-time setup
REM =========================================================================
:cmd_setup
call "%REPO_ROOT%\setup.bat"
exit /b !errorlevel!

REM =========================================================================
REM help — show usage
REM =========================================================================
:cmd_help
echo cargo-amiga.bat — Project management wrapper for Rust-for-AmigaOS4
echo.
echo Subcommands:
echo.
echo   new ^<name^> [--mode app^|driver]
echo       Create a new project from a template.
echo       --mode app      Application with clib4 and -lauto (default)
echo       --mode driver   Handler/driver — no CRT, no clib4
echo.
echo   build [^<project-path^>]
echo       Build a project (delegates to build.bat).
echo       If no path is given, uses the current directory.
echo.
echo   clean [^<project-path^>]
echo       Clean a project (delegates to build.bat ... clean).
echo       If no path is given, uses the current directory.
echo.
echo   run [^<project-path^>] [--target NAME] [--args "..."]
echo       Build, then deploy + launch the exe on a fleet target (QEMU or
echo       hardware) via the amiga-fleet CLI. Starts + stops QEMU around the run.
echo.
echo   test [^<project-path^>] --wait REGEX [--target NAME] [--args "..."] [--timeout N]
echo       Like 'run', but wait for REGEX in the target's serial output and exit
echo       non-zero if it is not seen within --timeout seconds (a headless smoke).
echo       Needs the amiga-fleet CLI: a sibling MCP-AmigaOS4 checkout, or set
echo       AMIGA_FLEET_MCP_HOST=path\to\MCP-AmigaOS4\host.
echo.
echo   setup
echo       Run first-time setup (delegates to setup.bat).
echo.
echo   help
echo       Show this message.
echo.
echo Examples:
echo.
echo   cargo-amiga.bat new my-cool-app
echo   cargo-amiga.bat new ram-handler --mode driver
echo   cargo-amiga.bat build my-cool-app
echo   cargo-amiga.bat run my-cool-app --target qemu
echo   cargo-amiga.bat test examples/test-harness --wait "51/51 tests passed"
echo   cargo-amiga.bat clean my-cool-app
echo   cargo-amiga.bat setup
exit /b 0
