#!/bin/bash
set -e
ARCH=${1:-$(uname -m)}
OUT=target/adzan-menubar

swiftc menubar/main.swift menubar/AppDelegate.swift menubar/SocketReader.swift \
    -o "$OUT" \
    -framework AppKit \
    -framework Foundation \
    -target "${ARCH}-apple-macosx12.0"

echo "Built: $OUT"
