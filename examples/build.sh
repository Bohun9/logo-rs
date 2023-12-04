#!/bin/bash

configs=(
    "fern 800 600"
    "rectangles 1920 1080"
    "text_spiral 1920 1080"
)

for config in "${configs[@]}"
do
    file_name=$(echo "${config}" | cut -d' ' -f1)
    width=$(echo "${config}" | cut -d' ' -f2)
    height=$(echo "${config}" | cut -d' ' -f3)

    echo "running ${file_name}.logo ..."
    cargo run -q --release -- "${file_name}.logo" "${file_name}.svg" "${width}" "${height}"
done

