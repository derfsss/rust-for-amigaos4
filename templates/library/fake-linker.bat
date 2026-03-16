@echo off
REM Fake linker — stub that creates an empty output file.
:loop
if "%~1"=="" goto end
if "%~1"=="-o" (type nul > "%~2" & goto end)
shift
goto loop
:end
