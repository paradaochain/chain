#!/usr/bin/env node

const { Keyring } = require('@polkadot/keyring');
const { ApiPromise, WsProvider } = require("@polkadot/api");
const path = require("path")
const fs = require("fs")
const { ContractPromise } = require('@polkadot/api-contract');

// yarn createdao 5CBhT1XmQVutFogBrV81VTLomfWWXAD1qjV7WVZgLNTieY85 "working club" asdfajfsdlkajf 0 1000000


async function main () {

  const ip = "127.0.0.1"
  const port = "9944"

  const factoryContractAddress = process.argv[2]
  const name = process.argv[3]
  const metadata = process.argv[4]
  const ty = parseInt(process.argv[5])
// 			name: String,
// 			metadata_url: String,
// 			ty: DaoType 0/ 1 ,
// 			joining_fee: Balance,

  console.log("factory address at : ", factoryContractAddress);
  console.log("name : ", name);
  console.log("metadata : ", metadata);
  console.log("ty: ", ty);

  const factory_metadata= fs.readFileSync(path.resolve(__dirname,
		"../target/ink/factory/metadata.json")).toString();


  const wsProvider = new WsProvider(`ws://${ip}:${port}`);

  const api = await ApiPromise.create({
    provider: wsProvider,
  });

  const keyring = new Keyring({ type: 'sr25519' });
  const alice = keyring.addFromUri('//Alice');
	let daoAddress = null;

  const contract = new ContractPromise(api, factory_metadata, factoryContractAddress);
  // maximum gas to be consumed for the call. if limit is too small the call will fail.
  const gasLimit = 1000000n * 1000000n;
  // a limit to how much Balance to be used to pay for the storage created by the contract call
  // if null is passed, unlimited balance can be used
  const storageDepositLimit = null

  // (We perform the send from an account, here using Alice's address)
	let timestamp = Date.now();
	 await contract.tx
	   .createDao({ storageDepositLimit, gasLimit }, name, metadata, ty, 100000, null,
		Math.floor(timestamp/1000))
	   .signAndSend(alice, result => {
	     if (result.status.isInBlock) {
           result.events.forEach( ({ event: { data, method, section }, phase }) => {
             console.log('\t', phase.toString(), `: ${section}.${method}`, data.toString());
			 if (section == "contracts" && method == "Instantiated"){
      			daoAddress = data[1].toString();
			}
           });
	     } else if (result.status.isFinalized) {
			 console.log('DAO address: ', daoAddress);
			 process.exit(0)
	     }
	   });


}

main().catch(console.error)
