[<img src = "https://raw.githubusercontent.com/Trinkler/brand/master/assets/katal/banner.png" width = "100%">](https://katalchain.com)

# katal-chain [![buddy pipeline](https://app.buddy.works/trinkler/katal-chain/pipelines/pipeline/195629/badge.svg?token=3a967ff05891e3690d97195573654d05994285b9798ed78a42d7178be77fa4c3 "buddy pipeline")](https://app.buddy.works/trinkler/katal-chain/pipelines/pipeline/195629)

> [Katal](https://katalchain.com)  implements a [taxonomy](./taxonomy.md) of financial contracts. These financial contracts are homogenous in terms of the cash flow patterns they express. The taxonomy can be seen as a _standard framework_ with which nearly every financial contract we see in _finance_ today can be described. View on [Telemetry](https://telemetry.polkadot.io/#list/Katal).

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
cargo install --locked --path . # Build and install native binaries
katal # Synchronize chain data
```

### Custom Modules

-   [Fixed-point arithmetic](https://github.com/Trinkler/katal-chain/tree/master/modules/reals)
-   [Time in ISO8601 format](https://github.com/Trinkler/katal-chain/tree/master/modules/time)
-   [Permissioned Oracle](https://github.com/Trinkler/katal-chain/tree/master/modules/oracle)
-   [Algorithmic Contract Types Unified Standard](https://github.com/Trinkler/katal-chain/tree/master/modules/actus)

### Additional Resources

-   [Read Research Papers](https://github.com/Trinkler/katal-research)
-   [Subscribe to Newsletter](https://software.us19.list-manage.com/subscribe?u=48964a7b4b5e9480604838bf2&id=982968577c)
-   [Follow on Twitter](https://twitter.com/katalchain)
-   [Join on Telegram](https://t.me/katalchain)
