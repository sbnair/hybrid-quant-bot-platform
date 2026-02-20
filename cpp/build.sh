#!/bin/bash
mkdir -p build
cd build
cmake ..
make
echo "Library built at build/liborderrouter.so"
