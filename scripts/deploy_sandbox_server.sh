#!/bin/sh

. ./scripts/env.sh && cargo run --no-default-features --features "server" -- 0.0.0.0:4000 --cert certs/certificate.crt --key certs/private.key
