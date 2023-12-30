# axum-lambda

## Prerequisites

-   [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
-   [cargo-lambda](https://github.com/cargo-lambda/cargo-lambda#installation)

```sh
$ cargo --version
cargo 1.74.1 (ecb9851af 2023-10-18)
```

```sh
$ cargo lambda --version
cargo-lambda 1.0.0 (dc04c1f 2023-11-13Z)
```

## Running

`make`

```sh
$ make
cargo lambda watch
 INFO invoke server listening on [::]:9000
 INFO starting lambda function function="_" manifest="Cargo.toml"
    Finished dev [unoptimized + debuginfo] target(s) in 0.21s
     Running `target/debug/axum-lambda`
 INFO 🚀 Starting server on http://localhost:9000
```

## Testing

`make test`

```sh
$ make test
cargo test
    Finished test [unoptimized + debuginfo] target(s) in 0.18s
     Running unittests src/main.rs (target/debug/deps/axum_lambda-49a197ff88995d46)

running 1 test
test tests::ping ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Further Reading / Useful Links

-   example (aws-lambda-rust-runtime): https://github.com/awslabs/aws-lambda-rust-runtime/blob/main/examples/http-axum/src/main.rs
-   example (axum): https://github.com/tokio-rs/axum/blob/main/examples/testing/src/main.rs
