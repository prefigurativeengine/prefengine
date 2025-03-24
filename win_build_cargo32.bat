@echo off

REM prefer DLLs for now
set "RUSTFLAGS=-C prefer-dynamic"

REM get needed files over to exe cwd
copy "pref-ret\retapi.py" "build\i686-pc-windows-msvc"
copy "data\reticulum_dummy_config.conf" "build\i686-pc-windows-msvc"
copy "data\self_peer.dummy.json" "build\i686-pc-windows-msvc"

cargo build --workspace --target i686-pc-windows-msvc --target-dir build
