@echo off
REM setup.bat — One-time setup for Rust-for-AmigaOS4 (Windows)
REM
REM Installs Rust nightly, rust-src component, and pulls the Docker image.
REM Run once before your first build.

echo === Rust for AmigaOS 4 — Setup ===
echo.

REM 1. Check rustup
where rustup >nul 2>&1
if errorlevel 1 (
    echo ERROR: rustup not found. Install from https://rustup.rs/
    exit /b 1
)

REM 2. Install nightly toolchain
echo [1/3] Installing Rust nightly toolchain...
rustup toolchain install nightly
echo   Done.

REM 3. Install rust-src (needed for build-std)
echo [2/3] Installing rust-src component...
rustup component add rust-src --toolchain nightly
echo   Done.

REM 4. Pull Docker image (via WSL)
echo [3/3] Pulling AmigaOS cross-compiler Docker image via WSL...
wsl sh -c "docker pull walkero/amigagccondocker:os4-gcc11"
echo   Done.

echo.
echo === Setup complete ===
echo.
echo Next steps:
echo   cd examples\hello
echo   build.bat
