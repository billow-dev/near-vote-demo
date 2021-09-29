# vote-demo
这是一个基于near的投票合约，在这里你可以创建投票的议题进行投票。

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
