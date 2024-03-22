#!/bin/bash

# Iterate over each .ppm file in the test-data directory
for file in test-data/tc_*; do
    # Extract the xxxx portion from the basename
    rawname=$(echo "$file" | sed 's/^tc_//')

    # Perform conversion using ImageMagick's convert command
    convert "$file/image.ppm" -monochrome -define tiff:photometric=min-is-white -depth 1 "$file/raw.tiff"

    tiffcp -c g4 "$file/raw.tiff" "$file/image.tiff"

    tiffinfo -d -r "$file/image.tiff" | grep -A 1 'Strip 0' | tail -n 1 | sed 's/.*: //' | xxd -r -p > "$file/image.dat"
done
