`RUSTFLAGS='-C target-cpu=neoverse-n1' cross build --release --target aarch64-unknown-linux-gnu`
https://github.com/awslabs/aws-lambda-rust-runtime/issues/354

<img width="1624" alt="Screen Shot 2021-12-21 at 1 22 25 AM" src="https://user-images.githubusercontent.com/11553110/146799573-e9c38c78-2e5c-47ad-9a90-17562dd0c137.png">

# M1 Mac
https://github.com/awslabs/aws-lambda-rust-runtime#aws-cli
Building on MacOS Using Docker

```
docker run \
  --platform ${LAMBDA_ARCH} \
  --rm --user "$(id -u)":"$(id -g)" \
  -v "${PWD}":/usr/src/myapp -w /usr/src/myapp rust:${RUST_VERSION} \
cargo build --example --release --target ${RUST_TARGET}
```

`cp ./target/aarch64-unknown-linux-gnu/release/arm-test ./bootstrap && zip lambda.zip bootstrap && rm bootstrap`