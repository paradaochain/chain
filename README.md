# ParaDAO

_para dão - to give_

ParaDAO provides tools for communities with the same passion / shared interests to come together to organise amongst themselves to acheive a common goal.
The decentralised application that allows communities to create a DAO easily with out of the box governance models for managing:

-   DAO treasury (moving of funds to certain purposes: host an event (offchain), staking / swapping (onchain))
-   DAO details (Updating some onchain /decentalised storage data of the DAO)
-   Proxy execution ( Enable the DAO to take part in other DApps, for example joining / voting in
    another DAO )
-   Oracle for prediction market creation and referee

ParaDAO is not only a tool, it in itself is a DAO and members are the DAOs created.
This ensures that ParaDAO continues to provide useful features to support the community.

For details, please see our [docs].

[docs]: https://paradaochain.github.io/docs/

## Hack

### Docker

```sh
# Run network
docker-compose --file docker-compose-xc.yml up

# Stop and clear
docker-compose --file docker-compose-xc.yml down -v && ./scripts/clear-all.sh
```

_Note: This may take a few minutes for Parachain to start producing blocks as it gets registered_

The relavant ports are:

-   Paradao ws: 9944
-   Paradao rpc: 9933
-   Relay ws: 6644

You can then deploy contracts at https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9944#/contracts
The package is contract_name.contract

#### Contracts

-   Go to `contracts/factory` and do `./build-all.sh`
-   Head to UI to first _upload_ dao wasm - `target/ink/dao/dao.wasm`.
-   Once the upload tx has been included in the block, obtain the code hash in the `Event` (under
-   Network: Chain Info) in the UI.
-   _upload and intantiate_ the factory contract - `target/ink/factory/factory.contract`, use the param
    0 and the previous dao contract code hash (`ty` will change the structure of daos)

### Local build collator

Requirements:

-   Node
-   Docker
-   Rust
-   jq
-   curl

#### 1. Build the collator for the parachain

```sh
# root dir
cargo build --release
```

#### 2. Run the relay chain

```sh
docker-compose --file docker-compose-xc-local.yml up
```

#### 3. Register and start parachain collator

_Note: Ensure that step 2 nodes are producing blocks_

```sh
	./scripts/local-run-para.sh
```

#### 4. Tear down

```sh
docker-compose --file docker-compose-xc-local.yml down -v && ./scripts/clear-all.sh
```

## Try it out!

1. Please fund yourself with paradao parachain tokens (to create, join, propose and vote) with `wss:://paradao.space/paradao`
2. Also fund youself with Zeitgeist dev tokens (to create prediction market and buy OST) with `wss://paradao.space/zg`
3. Head over to paradao.space 🚀

_Note: the Paradao Parachain has a block time of 12 seconds, please be patient_
_Note: if you made the proposal, your vote has been counted and you cannot use the same account to vote again_
