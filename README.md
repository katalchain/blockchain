[<img src = "https://raw.githubusercontent.com/Trinkler/brand/master/assets/katal/banner.png" width = "100%">](https://katalchain.com)

# katal-chain [![buddy pipeline](https://app.buddy.works/trinkler/katal-chain/pipelines/pipeline/195629/badge.svg?token=3a967ff05891e3690d97195573654d05994285b9798ed78a42d7178be77fa4c3 "buddy pipeline")](https://app.buddy.works/trinkler/katal-chain/pipelines/pipeline/195629)

> [Katal](https://katalchain.com) is a domain-specific blockchain creating a deterministic financial paradigm.

> Katal implements a [classification](https://www.actusfrf.org/taxonomy) of _contract types_ which are mutual agreements between counterparties to exchange cash flows. Nearly every [financial instrument](https://en.wikipedia.org/wiki/Financial_instrument) can be broken down into contract types.

> Katal is based on [Substrate](https://github.com/paritytech/substrate) and aims to connect to the multichain framework [Polkadot](https://polkadot.network) to interact with assets
> of [other connected blockchains](https://forum.web3.foundation/t/teams-building-on-polkadot/67) as well as to allow other connected blockchains to have access to contract types built using Katal. View on [Telemetry](https://telemetry.polkadot.io/#list/Katal).

### Setup

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh # Install Rustup
git clone git@github.com:Trinkler/katal-chain.git
cd katal-chain/
./scripts/init.sh # Initialize WASM build environment
```

### Develop

```sh
./scripts/build.sh # Build wasm binaries
cargo build # Build native binaries in debug mode
./target/debug/katal purge-chain --dev # Remove the whole chain data
./target/debug/katal --dev # Run chain in dev mode
```

### Install

```sh
cargo install --locked --path . --force # Build and install native binaries
katal # Synchronize chain data
```

### Usage

To access the [Katal Testnet](https://telemetry.polkadot.io/#list/Katal) using the great [Polkadot JS Apps Interface](https://polkadot.js.org/apps/#/explorer) do the following:

1. In [Settings](https://polkadot.js.org/apps/#/settings) tab under the `General` section select `wss://endpoint.katalchain.com` as remote endpoint.
2. In [Settings](https://polkadot.js.org/apps/#/settings) tab under the `Developer` section copy paste the [custom types definitions](https://raw.githubusercontent.com/Trinkler/katal-chain/master/interface/types.json) into the interface and click the "Save" button.

All done you are now able to for example deploy a contract under the [Extrinsics](https://polkadot.js.org/apps/#/extrinsics) tab using the `contracts` module.

### Custom Modules

- [Contracts](https://github.com/Trinkler/katal-chain/tree/master/modules/contracts)
- [Assets](https://github.com/Trinkler/katal-chain/tree/master/modules/assets)
- [Structures](https://github.com/Trinkler/katal-chain/tree/master/modules/structures)
  - [Safe fixed-point arithmetic](https://github.com/Trinkler/katal-chain/tree/master/modules/structures/src/reals.rs)
  - [Time in ISO8601 format](https://github.com/Trinkler/katal-chain/tree/master/modules/structures/src/time.rs)
  - [Priority queue using a binary heap](https://github.com/Trinkler/katal-chain/tree/master/modules/structures/src/min_heap.rs)
- [Oracle](https://github.com/Trinkler/katal-chain/tree/master/modules/oracle)

### Additional Resources

- [Read Research Papers](https://github.com/Trinkler/katal-research)
- [Subscribe to Newsletter](https://software.us19.list-manage.com/subscribe?u=48964a7b4b5e9480604838bf2&id=982968577c)
- [Follow on Twitter](https://twitter.com/katalchain)
- [Follow on Medium](https://medium.com/@katalchain)
- [Join on Telegram](https://t.me/katalchain)
