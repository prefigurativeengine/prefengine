#!/bin/bash

cargo build --workspace

TARGET_DIR="target/debug"

# get needed files over to exe cwd
cp "pref-ret/retapi.py" "$TARGET_DIR"
if [ "$1" == "overwrite_data" ]; then
    cp "data/reticulum_dummy_config.conf" "$TARGET_DIR"
    cp "data/self_peer.dummy.json" "$TARGET_DIR"
    cp "data/expected_temps.json" "$TARGET_DIR"
    cp "data/peers.json" "$TARGET_DIR"
fi
