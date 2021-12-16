#!/bin/bash

echo "\nBuilding file at: ${1}"
rm -rf output
rustc ./$1 -o output

echo "=====================\n\n\n"
if [ -f output ]; then
    echo "Running file at: ${1}"
    ./output
else
    echo "'output' File not found"
fi