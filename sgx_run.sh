#!/bin/bash
make && \
cd bin
./app &

sleep 1

cd ../client
cargo run --release

process_id=`/bin/ps | grep "app" | awk '{print $1}'`
kill -9 $process_id