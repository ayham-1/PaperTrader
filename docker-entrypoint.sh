#!/bin/sh

[ $1 == "Debug" ] && ./build.sh
[ $1 == "Release" ] && ./buildRelease.sh
[ $1 == "Debug" ] && cd build
[ $1 == "Release" ] && cd buildRelease 

cd ..

[ $1 == "Debug" ] && chmod -R 777 build lib bin
[ $1 == "Release" ] && chmod -R 777 buildRelease lib bin
