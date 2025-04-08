@echo off

cargo build --workspace --target x86_64-pc-windows-msvc --target-dir build

REM get needed files over to exe cwd
copy "pref-ret\retapi.py" "build\x86_64-pc-windows-msvc\debug"
if NOT "%1"=="no_dcpy" (
    copy "data\reticulum_dummy_config.conf" "build\i686-pc-windows-msvc\debug"
    copy "data\self_peer.dummy.json" "build\i686-pc-windows-msvc\debug"
    copy "data\expected_temps.json" "build\i686-pc-windows-msvc\debug"
    copy "data\peers.json" "build\i686-pc-windows-msvc\debug"
)
