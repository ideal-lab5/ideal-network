# Ideal Network Node

This repository contains implementations of the Ideal Network parachain node.

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

**Benchmarks**

Build with benchmarks using:
``` sh
cargo build --release --features runtime-benchmarks
```

and run them with:
``` 
# list all benchmarks
./target/release/ideal-nw-node benchmark pallet --chain dev --pallet "*" --extrinsic "*" --repeat 0
# benchmark the drand pallet
./target/release/ideal-nw-node benchmark pallet \
    --chain dev \
    --wasm-execution=compiled \
    --pallet pallet_drand \
    --extrinsic "*" \
    --steps 50 \
    --repeat 20 \
    --output <output_file.rs>
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