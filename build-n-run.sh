#!/bin/bash

echo "\nBuilding file at: ${1}"
rm -rf output
rustc ./$1 -o output

if [ -f output ]; then
    echo "Running file at: ${1}"
    echo "=====================\n\n\n"
    ./output
else
    echo "'output' File not found"
fi