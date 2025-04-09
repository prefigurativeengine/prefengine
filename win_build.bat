@echo off

cargo build --workspace

set TARGET_DIR="target\debug"

REM get needed files over to exe cwd
copy "pref-ret\retapi.py" %TARGET_DIR%
if "%1"=="overwrite_data" (
    copy "data\reticulum_dummy_config.conf" %TARGET_DIR%
    copy "data\self_peer.dummy.json" %TARGET_DIR%
    copy "data\expected_temps.json" %TARGET_DIR%
    copy "data\peers.json" %TARGET_DIR%
)
