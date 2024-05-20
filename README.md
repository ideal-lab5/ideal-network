# Ideal Network Node

This repository contains implementations of the ETF consensus mechanism and a substrate node that uses it.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="./resources/web3%20foundation_grants_badge_white.png">
  <img alt="This project is funded by the Web3 Foundation Grants Program" src="./resources/web3%20foundation_grants_badge_black.png">
</picture>


### Build

Use the following command to build the node without launching it:

```sh
cargo build --release
```

**Docker**

From the root directory, run:

``` sh
docker build .
```

### Testing

**Unit Tests**

``` sh
cargo test
```

**E2E Tests**

``` sh
cargo test --features e2e
```

**Benchmarks**

Build with benchmarks using:
``` sh
cargo build --release --features runtime-benchmarks
```

and run them with:
``` 
# list all benchmarks
./target/release/node benchmark pallet --chain dev --pallet "*" --extrinsic "*" --repeat 0
# benchmark the etf pallet
./target/release/node benchmark pallet \
    --chain dev \
    --wasm-execution=compiled \
    --pallet pallet_etf \
    --extrinsic "*" \
    --steps 50 \
    --repeat 20 \
    --output /pallets/etf/src/weight.rs
```