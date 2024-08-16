# Encryption to the Future Node

This repository contains implementations of the ETF consensus mechanism and a substrate node that uses it.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="./resources/web3%20foundation_grants_badge_white.png">
  <img alt="This project is funded by the Web3 Foundation Grants Program" src="./resources/web3%20foundation_grants_badge_black.png">
</picture>


## Build

Use the following command to build the node without launching it:

```sh
cargo build --release
```

**Docker**

From the root directory, run:

``` sh
docker build .
```

## Testing

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

## Local Development Chain

🧟 This project uses [Zombienet](https://github.com/paritytech/zombienet) to orchestrate the relaychain and parachain nodes.
You can grab a [released binary](https://github.com/paritytech/zombienet/releases/latest) or use an [npm version](https://www.npmjs.com/package/@zombienet/cli).

This project produces a IDN parachain node.
You still need a relaychain node - you can download the `polkadot`
(and the accompanying `polkadot-prepare-worker` and `polkadot-execute-worker`)
binaries from [Polkadot SDK releases](https://github.com/paritytech/polkadot-sdk/releases/latest).

Make sure to bring the `ideal-nw-node` - as well as `polkadot`, `polkadot-prepare-worker`, `polkadot-execute-worker`,
and `zombienet` - into `PATH` like so:

```sh
export PATH="./target/release/:$PATH"
```

This way, we can conveniently use them in the following steps.

👥 The following command starts a local development IDN chain, with a single relaychain node and a single parachain collator:

```sh
zombienet --provider native spawn ./zombienet.toml

# Alternatively, the npm version:
npx --yes @zombienet/cli --provider native spawn ./zombienet.toml
```

Once it's running you should insert the Drand pallet's authority keys. For Alice it can be done by running the following command:
```sh
chmod +x insert_alice_drand_key.sh
./insert_alice_drand_key.sh
```

Then you can interact with the relaychain on `ws://127.0.0.1:9944` and with the parachain on `ws://127.0.0.1:9988`.