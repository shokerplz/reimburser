#!/usr/bin/env bash
set -euo pipefail

# Builds PDFium as a static library (libpdfium.a) for the current platform.
# Requires: git, python3, ninja (or will use depot_tools' ninja)
# Output: lib/libpdfium.a
#
# This only needs to be run once per platform. The resulting libpdfium.a
# is then used by cargo build via PDFIUM_STATIC_LIB_PATH.

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
LIB_DIR="$SCRIPT_DIR/lib"
STATIC_LIB="$LIB_DIR/libpdfium.a"

if [[ -f "$STATIC_LIB" ]]; then
  echo "libpdfium.a already exists at $STATIC_LIB — skipping build."
  echo "Delete it and re-run this script to rebuild."
  exit 0
fi

WORK_DIR=$(mktemp -d)
echo "Working in $WORK_DIR"

cleanup() {
  echo ""
  echo "Build succeeded. Deleting temporary build directory"
  rm -rf $WORK_DIR
  echo "Done"
}
trap cleanup EXIT

# Get depot_tools
if ! command -v gclient &>/dev/null; then
  echo "==> Fetching depot_tools..."
  git clone https://chromium.googlesource.com/chromium/tools/depot_tools.git "$WORK_DIR/depot_tools"
  export PATH="$WORK_DIR/depot_tools:$PATH"
fi

# Fetch PDFium source
echo "==> Fetching PDFium source (this downloads several GB)..."
cd "$WORK_DIR"
gclient config --unmanaged https://pdfium.googlesource.com/pdfium.git
gclient sync

# Configure build
echo "==> Configuring build..."
cd "$WORK_DIR/pdfium"

mkdir -p out/Release

cat > out/Release/args.gn <<'GN_ARGS'
is_debug = false
is_component_build = false
pdf_is_standalone = true
pdf_is_complete_lib = true
pdf_enable_v8 = false
pdf_enable_xfa = false
pdf_bundle_freetype = true
use_custom_libcxx = false
use_sysroot = false
use_glib = false
is_clang = false
treat_warnings_as_errors = false
GN_ARGS

gn gen out/Release

# Build
echo "==> Building PDFium (this may take a while)..."
ninja -C out/Release pdfium

# Copy result
mkdir -p "$LIB_DIR"
cp out/Release/obj/libpdfium.a "$LIB_DIR/"

echo "e
echo "==> Done! Static library installed to $STATIC_LIB"
echo "    You can now build the project with: cargo build"
