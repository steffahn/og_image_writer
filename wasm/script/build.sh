#!/bin/sh

wasm-pack build --target web --release
sed -i "" -e 's/"name": "wasm"/"name": "og_image_writer"/' pkg/package.json
