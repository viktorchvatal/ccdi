#!/bin/sh
set -e

CLIENT_DIR=../ccdi-web-client
CLIENT_DIST=$CLIENT_DIR/dist
DST_DIR=./src/static
sh -c "cd $CLIENT_DIR; cargo build --release"

WASM_FILE_SRC=$CLIENT_DIST/`ls -1 $CLIENT_DIST | grep ccdi-web-client | grep wasm`
JS_FILE_SRC=$CLIENT_DIST/`ls -1 $CLIENT_DIST | grep ccdi-web-client | grep js`
CSS_FILE_SRC=$CLIENT_DIST/`ls -1 $CLIENT_DIST | grep css`

WASM_FILE_DST=$DST_DIR/ccdi-web-client.wasm
JS_FILE_DST=$DST_DIR/ccdi-web-client.js
CSS_FILE_DST=$DST_DIR/ccdi-web-client.css

echo "Creating symlink $WASM_FILE_DST -> $WASM_FILE_SRC"
ln -sr $WASM_FILE_SRC $WASM_FILE_DST
echo "Creating symlink $JS_FILE_DST -> $JS_FILE_SRC"
ln -sr $JS_FILE_SRC $JS_FILE_DST
echo "Creating symlink $CSS_FILE_DST -> $CSS_FILE_SRC"
ln -sr $CSS_FILE_SRC $CSS_FILE_DST