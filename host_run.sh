#!/bin/bash
cd io-uring
cargo run --example tcp_echo &

sleep 1

cd ../client
cargo run --release

process_id=`/bin/ps | grep "tcp_echo" | awk '{print $1}'`
kill -9 $process_id