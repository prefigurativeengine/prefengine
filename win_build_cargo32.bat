@echo off

REM Check if the argument is provided
if "%~1" == "" (
    echo Please provide the name of the Cargo package.
    exit /b
)

REM Set the environment variable CARGO_TARGET_DIR
set "CARGO_TARGET_DIR=%cd%\%1\target\32"

cd %1

REM Call the cargo build command
cargo build -p %1
