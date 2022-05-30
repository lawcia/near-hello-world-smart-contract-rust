# NEAR Hello World Smart Contract

This project consists of two applications

1. A Rust SDK Smart Contract
2. A REACT frontend

## Requirements

1. Docker
2. A testnet account on Near, you can create one [here](https://wallet.testnet.near.org/)

## Commands

Build the contract

```
docker-compose run contract cargo build --target wasm32-unknown-unknown --release
```

Deploy contract

```
docker-compose run contract /bin/bash
```

Then use the terminal to login

```
near login
```

Create a sub account

```
near create-account YOUR_CONTRACT_NAME.YOUR_ACCOUNT.testnet --masterAccount YOUR_ACCOUNT.testnet

```

Then enter this command to deploy your contract

```
near deploy --wasmFile target/wasm32-unknown-unknown/release/YOUR_PROJECT.wasm --accountId YOUR_ACCOUNT_HERE

```

Test

```
docker-compose run contract cargo test -- --nocapture
```


