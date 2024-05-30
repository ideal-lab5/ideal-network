# Encryption to the Future Node

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

## WIP Parachain 

https://docs.substrate.io/tutorials/build-a-parachain/prepare-a-local-relay-chain/

https://docs.substrate.io/tutorials/build-a-parachain/connect-a-local-parachain/

```sh
./target/release/ideal-nw-node build-spec --disable-default-bootnode > plain-ideal-nw-chainspec.json
```

```sh
./target/release/ideal-nw-node build-spec --chain plain-ideal-nw-chainspec.json --disable-default-bootnode --raw > raw-ideal-nw-chainspec.json
```

generate node keys
```sh
./target/release/polkadot key generate-node-key --base-path ./target/tmp/relay/alice
```

launch node 1
```sh
./target/release/polkadot \                                               
--alice \
--validator \
--base-path ./target/tmp/relay/alice \
--chain ./target/tmp/raw-local-chainspec.json \
--port 30333 \
--rpc-port 9944 \
--insecure-validator-i-know-what-i-do
```

launch node 2
```sh
./target/release/polkadot \
--bob \
--validator \
--base-path ./target/tmp/relay/bob \
--chain ./target/tmp/raw-local-chainspec.json \
--port 30334 \
--rpc-port 9945 \
--insecure-validator-i-know-what-i-do \
--bootnodes /ip4/127.0.0.1/tcp/30333/p2p/[NODE-1-ID]
```

Copy Alice keys over from Relay to Parachain. (?)

Launch Parachain
```sh
 ./target/release/ideal-nw-node \
--alice \
--collator \
--force-authoring \
--chain raw-ideal-nw-chainspec.json \
--base-path ./target/tmp/ideal-nw/alice \
--port 40333 \
--rpc-port 8844 \
-- \
--execution wasm \
--chain ../polkadot-sdk/target/tmp/raw-local-chainspec.json \
--port 30343 \
--rpc-port 9977 \
--bootnodes /ip4/127.0.0.1/tcp/30333/p2p/[NODE-1-ID]
```