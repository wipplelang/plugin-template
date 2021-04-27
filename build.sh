#!/bin/bash

set -e

cargo build --release

if [ -f target/release/libplugin.so ]; then
    cp target/release/libplugin.so project/plugin.wplplugin
elif [ -f target/release/libplugin.dylib ]; then
    cp target/release/libplugin.dylib project/plugin.wplplugin
else
    echo "Error: Target not supported"
    exit 1
fi

zip -rq plugin.zip project
