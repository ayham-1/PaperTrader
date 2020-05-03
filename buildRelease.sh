#!/bin/sh

mkdir -p buildRelease && cd buildRelease
cmake -GNinja --config=Release ..
ninja -j$(nproc)
cd ..
