# io_uring-rust-sgx-example
based on rust-sgx-sdk


## io_uring rust-sgx-sdk sample
Modified from [https://github.com/apache/incubator-teaclave-sgx-sdk/tree/master/samplecode/hello-rust](https://github.com/apache/incubator-teaclave-sgx-sdk/tree/master/samplecode/hello-rust).

Responsible for invoking io_uring-example in SGX. More specifically, porting the io-uring crate example tcp_echo to rust-sgx-sdk samplecode.

Contains following directories:

- ./app
- ./bin
- ./enclave
- ./lib

## client crate
client app for tcp_echo example.

Contains following directories:
- ./client


## io-uring crate
Modified from [https://github.com/tokio-rs/io-uring](https://github.com/tokio-rs/io-uring) at commit _c812adbd917ad33132a760698f5d276b1365fbc6_ .

Support SGX and Linux.

when use in SGX, need specify: `default-features = false, features = [ "sgx-feature" ]`. And io-uring ocall is implemented in ./app/src/io_uring_ocall.

Contains following directories:

- ./io-uring

## untrusted_allocator crate
Support untrusted memory managemnet in SGX

Contains following directories:

- ./untrusted_allocator

## slab crate
Modified from [https://github.com/carllerche/slab](https://github.com/carllerche/slab) at commit _f1327f72fb14fb387e07a6123300eb7a9da7c4e8_

It's used by io_uring rust-sgx-sdk sample.

Support SGX and Linux.

when use in SGX, need specify: `default-features = false, features = [ "sgx-feature" ]`.

Contains following directories:

- ./slab
