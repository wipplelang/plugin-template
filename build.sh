#!/bin/bash

set -e

cargo build --release
cp target/release/libplugin.dylib plugin.wplplugin
