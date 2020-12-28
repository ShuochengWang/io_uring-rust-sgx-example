#!/bin/bash
# compile server and client
cd io-uring && cargo build --examples --release && \
cd ../client && cargo build --release && cd ..

# run server
./io-uring/target/release/examples/tcp_echo &

sleep 1

# run clients
./client/target/release/client &
./client/target/release/client &
./client/target/release/client

sleep 2
# kill server and clients
for pid in $(/bin/ps | grep "client" | awk '{print $1}'); do kill -9 $pid; done
for pid in $(/bin/ps | grep "tcp_echo" | awk '{print $1}'); do kill -9 $pid; done