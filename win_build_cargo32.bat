@echo off

REM Check if the argument is provided
@REM if "%~1" == "" (
@REM     echo Please provide the name of the Cargo package.
@REM     exit /b
@REM )

REM Set the environment variable CARGO_TARGET_DIR
REM set "CARGO_TARGET_DIR=%cd%\%1\target\32"
set "RUSTFLAGS=-C prefer-dynamic"

REM cd %1

REM Call the cargo build command
cargo build --workspace --target i686-pc-windows-msvc --target-dir build
