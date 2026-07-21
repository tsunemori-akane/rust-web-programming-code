#!/usr/bin/env bash
# File: ingress/scripts/run_server.sh

# navigate to directory
SCRIPTPATH="$( cd "$(dirname "$0")" ; pwd -P )"
cd $SCRIPTPATH

cd ../..
cd frontend
npm install
npm run build
cd ../ingress
cargo clean
export $(cat .env | xargs)
# Run with backtrace for better error info
RUST_BACKTRACE=1 cargo run
# cargo run