#!/bin/bash

echo "\nBuilding file at: ${1}"

rustc ./$1 -o output

echo "=====================\n\n\n"
./output