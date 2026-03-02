#!/bin/sh

ipng=./input.png

cat "${ipng}" |
    wasmtime \
        run \
        ./rs-png2exif.wasm \
        --input-img-bytes-max 16777216 |
    xxd
