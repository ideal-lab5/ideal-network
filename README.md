# Ideal Network Node

This repository contains implementations of the Ideal Network parachain node.

**WARNING**: This is a work in progress and is not ready for production use as it is based on an [unsafe version of the Drand pallet](https://github.com/ideal-lab5/pallet-drand/blob/main/docs/how_it_works.md#assumption-and-limitations).

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

1. This project uses [POP](https://onpop.io/) to orchestrate the relaychain and parachain nodes.
If you don't have it yet, install the [`pop` CLI tool](https://learn.onpop.io/v/cli/installing-pop-cli) to run the local development chain.

2. Run the following command to start a local development IDN chain, with two relaychain nodes and a single parachain collator:

```sh
pop up parachain -f ./network.toml
```
It should output something like this:
```
◇  🚀 Network launched successfully - ctrl-c to terminate
│  ⛓️ paseo-local
│       alice:
│         portal: https://polkadot.js.org/apps/?rpc=ws://127.0.0.1:51547#/explorer
│         logs: tail -f /var/folders/_y/qwer/T/zombie-asdf/alice/alice.log
│       bob:
│         portal: https://polkadot.js.org/apps/?rpc=ws://127.0.0.1:51550#/explorer
│         logs: tail -f /var/folders/_y/qwer/T/zombie-asdf/bob/bob.log
│  ⛓️ dev: 1000
│       collator-01:
│         portal: https://polkadot.js.org/apps/?rpc=ws://127.0.0.1:51553#/explorer
│         logs: tail -f /var/folders/_y/qwer/T/zombie-asdf/collator-01/collator-01.log
```
You can see here that the parachain node `collator-01` is running on `ws://127.0.0.1:51553`.
Take note of the parachain WS port, in this case `51553`, as you will need it to interact with the parachain in the next step.

3. Insert the Drand pallet's authority keys. For Alice it can be done by running the following command:
```sh
chmod +x insert_alice_drand_key.sh
./insert_alice_drand_key.sh <PARACHAIN_WS_PORT>
```

4. Done, you can now interact with the parachain using the "portal" link provided by the `pop` CLI tool (see the output from step 2).