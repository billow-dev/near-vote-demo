# vote-demo

## Project setup
```
yarn install
```

### Compiles and hot-reloads for development
```
yarn serve
```
## build and deploy

### build contract

build contract
```bash
cd contract
bash build.sh
near dev-deploy --wasmFile ../out/contract.wasm
```

new
```bash
near call $ID new --accountId $ID
```
