# Call other smart contract

这个 Demo 演示了在 substrate 的 ink 中，如何通过一个 contract 去调用另外一个 contract，这里被调用的 contract 我选择了 flipper contract。

参考了 https://github.com/paritytech/ink/tree/master/examples/delegator，但是这个版本在我这里没法上传成功，并且这个合约作为最简单的演示，有些复杂。

代码直接 clone。

## 问题

第一个需要注意的问题是，scale 和 scale-info 的版本。ink 的主干分支正处于活跃的开发之下，非常不稳定，目前主干的 scale 版本是 **2.0**，scale-info 版本 **0.5**。但是这个版本编译并不能通过，他的 `parity-scale-code` 需要定制的 [crate](https://github.com/paritytech/ink/tree/master/crates)。问题是，即使是这个定制的 crate，编译也会失败。

我自己尝试了下，scale **1.3**，scale-info **0.4.1** 比较稳定，能编译通过。

第二个问题是，可能因为 scale 版本原因，复杂一点的 `ink_stroage` 特性可能无法支持，这个我没深入研究，需要大家测一下。我自己试的时候，`PackedLayout` 和 `SpreadLayout` 会编译出问题。

## 流程

1. 首先运行 `./build.sh`，当然需要你有整套 ink 环境，ink 环境看看教程。
2. 运行 `canvas-node`，或者是一条支持 pallet-contracts 的 substrate 链。
3. 使用 [canvas-ui](https://paritytech.github.io/canvas-ui) 上传 .contract 文件。
4. 使用 [polkadot.js](https://polkadot.js.org/apps/)，或者自己本地运行一个
5. 开发者 -> 交易(extrinsics)，选择 `contracts` 模块，选择 `putCode`
6. 点击文件上传，把 flipper 合约的 wasm 文件上传上去
7. 打开浏览器 console，获取到 contract hash，类似于以下。这里说一下，polkadot app 的浏览很烂，所以要这种绕弯子的方法：

```json
signAndSend: status :: {"dispatchInfo":{"weight":1102599000,"class":"Normal","paysFee":"Yes"},"events":[{"phase":{"ApplyExtrinsic":1},"event":{"index":"0x0803","data":["0xbf1a3cf1c683f4809a2ece6c73f5a03473aed873ce82d01eaa9854f7529a0e7a"]},"topics":[]},{"phase":{"ApplyExtrinsic":1},"event":{"index":"0x0000","data":[{"weight":1102599000,"class":"Normal","paysFee":"Yes"}]},"topics":[]}],"status":{"InBlock":"0x258fdbb0754d455e3767c9dba6e423bea3f796a0985ea73717a0e35e4a6c8d54"}}
```

8. 在 canvas-ui 中点击部署，flipperCodeHash 使用前面得到的 flipper contract hash，填上去。
9. 现在可以调用了，delegator 能直接调用 flippr 的方法