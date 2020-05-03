#!/bin/sh

./clean.sh
docker build --tag papertraderbuilder .
docker run -v $(pwd):/PaperTrader papertraderbuilder Release
exit 0
