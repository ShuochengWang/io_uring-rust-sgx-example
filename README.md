# io_uring-rust-sgx-example
based on rust-sgx-sdk


## io_uring rust-sgx-sdk sample
Modified from [https://github.com/apache/incubator-teaclave-sgx-sdk/tree/master/samplecode/hello-rust](https://github.com/apache/incubator-teaclave-sgx-sdk/tree/master/samplecode/hello-rust)
Responsible for invoking io_uring-example in SGX.
Contains following directories:

- ./app
- ./bin
- ./enclave
- ./lib



## io-uring crate
Modified from [https://github.com/tokio-rs/io-uring](https://github.com/tokio-rs/io-uring) at commit _c812adbd917ad33132a760698f5d276b1365fbc6_
Support SGX and Linux.
when use in SGX, need specify: `default-features = false, features = [ "sgx-feature" ]`
Contains following directories:

- ./io-uring