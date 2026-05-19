@echo off
REM Fake linker for Windows host - captures args, creates empty output
REM Real linking done by ppc-amigaos-gcc in Docker
echo fake-linker: captured %* >> fake-linker.log
REM Find the -o argument and create an empty file there
:loop
if "%~1"=="" goto done
if "%~1"=="-o" (
    type nul > "%~2"
    goto done
)
shift
goto loop
:done
exit /b 0
