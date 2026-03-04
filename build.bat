@echo off
setlocal enabledelayedexpansion
REM build.bat — Build any project in this repository (Windows, via WSL+Docker)
REM
REM Usage:
REM   build.bat <project-path>           Build a project
REM   build.bat <project-path> clean     Clean a project
REM
REM Examples:
REM   build.bat examples\hello           Build the hello example
REM   build.bat examples\hello-driver    Build the hello-driver example
REM   build.bat examples\hello clean     Clean the hello example

if "%1"=="" (
    echo Usage: build.bat ^<project-path^> [clean]
    echo.
    echo Examples:
    echo   build.bat examples\hello
    echo   build.bat examples\hello-driver
    echo   build.bat examples\hello clean
    exit /b 1
)

set "PROJECT=%~1"
REM Normalise to forward slashes for Docker/WSL
set "PROJECT=%PROJECT:\=/%"

set "REPO_ROOT=%~dp0"
REM Remove trailing backslash
if "%REPO_ROOT:~-1%"=="\" set "REPO_ROOT=%REPO_ROOT:~0,-1%"

if not exist "%REPO_ROOT%\%~1\Cargo.toml" (
    echo ERROR: No Cargo.toml found in %~1
    exit /b 1
)

if "%2"=="clean" (
    echo Cleaning %PROJECT%...
    if exist "%REPO_ROOT%\%~1\target" rmdir /s /q "%REPO_ROOT%\%~1\target"
    if exist "%REPO_ROOT%\%~1\build" rmdir /s /q "%REPO_ROOT%\%~1\build"
    if exist "%REPO_ROOT%\%~1\fake-linker.log" del "%REPO_ROOT%\%~1\fake-linker.log"
    echo Done.
    exit /b 0
)

echo === Building: %PROJECT% ===
echo.

echo [1/2] Compiling Rust (host)...
cd "%REPO_ROOT%\%~1"
cargo +nightly build --release
if errorlevel 1 (
    echo ERROR: Rust compilation failed.
    exit /b 1
)
echo   Rust staticlib built.
echo.

echo [2/2] Linking (Docker via WSL with clib4-nightly overlay)...

REM Convert repo root to WSL path: W:\Code\foo -> /mnt/w/Code/foo
set "WSL_PATH=%REPO_ROOT:\=/%"
set "DRIVE_LETTER=%WSL_PATH:~0,1%"
REM Lowercase the drive letter
for %%a in (a b c d e f g h i j k l m n o p q r s t u v w x y z) do (
    if /i "%DRIVE_LETTER%"=="%%a" set "DRIVE_LOWER=%%a"
)
set "WSL_PATH=/mnt/%DRIVE_LOWER%/%WSL_PATH:~3%"

wsl sh -c "docker run --rm -v '%WSL_PATH%':/repo -w /repo/%PROJECT% walkero/amigagccondocker:os4-gcc11 sh -c 'cp -r /repo/clib4-nightly/lib/* /opt/ppc-amigaos/ppc-amigaos/SDK/clib4/lib/ && cp -r /repo/clib4-nightly/include/* /opt/ppc-amigaos/ppc-amigaos/SDK/clib4/include/ && make clean' && docker run --rm -v '%WSL_PATH%':/repo -w /repo/%PROJECT% walkero/amigagccondocker:os4-gcc11 sh -c 'cp -r /repo/clib4-nightly/lib/* /opt/ppc-amigaos/ppc-amigaos/SDK/clib4/lib/ && cp -r /repo/clib4-nightly/include/* /opt/ppc-amigaos/ppc-amigaos/SDK/clib4/include/ && make all'"

if errorlevel 1 (
    echo ERROR: Docker link failed.
    exit /b 1
)

echo.
echo === Build complete: %PROJECT% ===
