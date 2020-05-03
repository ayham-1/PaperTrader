#!/bin/sh

./clean.sh
docker build --tag papertraderbuilder .
docker run -v $(pwd):/PaperTrader papertraderbuilder Debug
exit 0
