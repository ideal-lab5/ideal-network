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

## Local Development Chain

🧟 This project uses [Zombienet](https://github.com/paritytech/zombienet) to orchestrate the relaychain and parachain nodes.
You can grab a [released binary](https://github.com/paritytech/zombienet/releases/latest) or use an [npm version](https://www.npmjs.com/package/@zombienet/cli).

This template produces a parachain node.
You still need a relaychain node - you can download the `polkadot`
(and the accompanying `polkadot-prepare-worker` and `polkadot-execute-worker`)
binaries from [Polkadot SDK releases](https://github.com/paritytech/polkadot-sdk/releases/latest).

Make sure to bring the parachain node - as well as `polkadot`, `polkadot-prepare-worker`, `polkadot-execute-worker`,
and `zombienet` - into `PATH` like so:

```sh
export PATH="./target/release/:$PATH"
```

This way, we can conveniently use them in the following steps.

👥 The following command starts a local development chain, with a single relay chain node and a single parachain collator:

```sh
zombienet --provider native spawn ./zombienet.toml

# Alternatively, the npm version:
npx --yes @zombienet/cli --provider native spawn ./zombienet.toml
```

## How to run IDL NW Parachain 

https://docs.substrate.io/tutorials/build-a-parachain/prepare-a-local-relay-chain/

https://docs.substrate.io/tutorials/build-a-parachain/connect-a-local-parachain/

1. Build the relay chain and parachain
```sh
cd polkadot-sdk
cargo build --release
cd ../ideal-network
cargo build --release
```

2. Generate node keys for Alice and Bob on the relay chain
```sh
./polkadot-sdk/target/release/polkadot key generate-node-key --base-path ./polkadot-sdk/target/tmp/relay/alice
./polkadot-sdk/target/release/polkadot key generate-node-key --base-path ./polkadot-sdk/target/tmp/relay/bob
```

3. Copy paste the generated Alice keys to the parachain
```sh
cp ./polkadot-sdk/target/tmp/relay/alice/chains/polkadot/network/secret_ed25519 \
./ideal-network/target/tmp/ideal-nw/alice/chains/local_testnet/network/secret_ed25519
```

4. Download the raw chainspec file
https://docs.substrate.io/assets/tutorials/relay-chain-specs/raw-local-chainspec.json
and save it to `polkadot-sdk/target/tmp/raw-local-chainspec.json`.
Change the `name` field to "Polkadot Local Testnet" and the `id` field to `polkadot`.

5. Run Alice validator on the relay chain
```sh
cd polkadot-sdk

./target/release/polkadot \                                                                                                                   
--alice \
--validator \
--base-path ./target/tmp/relay/alice \
--chain ./target/tmp/raw-local-chainspec.json \
--port 30333 \
--rpc-port 9944 \
--insecure-validator-i-know-what-i-do
```
And save the node id for Alice.

6. Run Bob validator on the relay chain
```sh
./target/release/polkadot \                                                                                                                   
--bob \  
--validator \
--base-path ./target/tmp/relay/bob \  
--chain ./target/tmp/raw-local-chainspec.json \
--port 30334 \
--rpc-port 9945 \
--insecure-validator-i-know-what-i-do \
--bootnodes /ip4/127.0.0.1/tcp/30333/p2p/[alice-node-id]
```
*Both validators should be running and finalizing relay chain blocks now.*

7. Reserve a parachain identifier:
- Open https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9944 in a browser.
- Go to Network > Parachains then to Parathreads.
- Click `+ ParaId` and reserve the ID 2000 for the parachain.
- Submit the transaction

8. Generate the chainspecs for the parachain
```sh
cd ../ideal-network
./target/release/ideal-nw-node build-spec --disable-default-bootnode > plain-ideal-nw-chainspec.json
./target/release/ideal-nw-node build-spec --chain plain-ideal-nw-chainspec.json --disable-default-bootnode --raw > raw-ideal-nw-chainspec.json
```

9. Generate genesis state and wasm blob for the parachain
```sh
./target/release/ideal-nw-node export-genesis-state --chain raw-ideal-nw-chainspec.json ideal-nw-2000-genesis-state
./target/release/ideal-nw-node export-genesis-wasm --chain raw-ideal-nw-chainspec.json ideal-nw-2000-wasm
```

10. Run the parachain collator
```sh
./target/release/ideal-nw-node \
--alice \
--collator \
--force-authoring \
--base-path ./target/tmp/ideal-nw/alice \
--port 40333 \
--rpc-port 8844 \
-- \
--execution wasm \
--chain ../polkadot-sdk/target/tmp/raw-local-chainspec.json \
--port 30343 \
--rpc-port 9977 \
--bootnodes /ip4/127.0.0.1/tcp/30333/p2p/[alice-node-id]
```
*The collator should be creating blocks, but not finalizing them yet.*

11. Register the parachain
- Open https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9944 in a browser.
- Go to Developer > Sudo
- Select `paraSudoWrapper` and `sudoScheduleParaInitialize` with:
  - `paraId`: 2000
  - `genesisHead`: upload the `ideal-nw-2000-genesis-state` file
  - `validationCode`: upload the `ideal-nw-2000-wasm` file
  - `paraKind`: Yes
  - Submit the transaction

12. Wait for the parachain to be registered and start finalizing blocks.

*You can check this on your parachain collator terminal.*