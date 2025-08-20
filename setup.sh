#!/usr/bin/env bash
set -euo pipefail

PDF_LIB="./lib/libpdfium.so"

if [[ ! -f "$PDF_LIB" ]]; then
  TMPDIR=$(mktemp -d)
  trap 'rm -rf "$TMPDIR"' EXIT
  wget -q -O "$TMPDIR/pdfium.tgz" "https://github.com/bblanchon/pdfium-binaries/releases/download/chromium%2F7350/pdfium-mac-arm64.tgz"
  tar -xzf "$TMPDIR/pdfium.tgz" -C "$TMPDIR"
  mkdir -p ./lib
  cp "$TMPDIR/lib/libpdfium.so" ./lib/
fi
