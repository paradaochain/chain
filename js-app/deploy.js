#!/usr/bin/env node

const { Keyring } = require('@polkadot/keyring');
const { ApiPromise, WsProvider } = require("@polkadot/api");
const path = require("path")
const fs = require("fs")
const { CodePromise } = require('@polkadot/api-contract');



async function main () {
  const ip = "127.0.0.1"
  const port = "9944"

  const dao_wasm = fs.readFileSync(path.resolve(__dirname,
		"../target/ink/dao/dao.wasm")).toString('hex');
  const factory_wasm= fs.readFileSync(path.resolve(__dirname,
		"../target/ink/factory/factory.wasm"));
  const factory_metadata= fs.readFileSync(path.resolve(__dirname,
		"../target/ink/factory/metadata.json")).toString();


  const wsProvider = new WsProvider(`ws://${ip}:${port}`);

  const api = await ApiPromise.create({
    provider: wsProvider,
  });

  const keyring = new Keyring({ type: 'sr25519' });
  const alice = keyring.addFromUri('//Bob');
  let dao_code_stored = null;

    await api.tx.contracts.uploadCode("0x"+dao_wasm, null)
      .signAndSend(alice, async ({ events = [], status }) => {
     	console.log('\n\n Uploading DAO: Transaction status:', status.type);

        if (status.isInBlock) {
          console.log('Included at block hash', status.asInBlock.toHex());
          console.log('Events:');

          events.forEach( ({ event: { data, method, section }, phase }) => {
            console.log('\t', phase.toString(), `: ${section}.${method}`, data.toString());
      		if (section == "contracts" && method == "CodeStored"){
      			dao_code_stored = data[0].toString();
      		}
          });

        } else if (status.isFinalized) {
            console.log('Finalized block hash for DAO code', status.asFinalized.toHex());

			if (!dao_code_stored) {
				console.log("DAO Code not set");
				process.exit(1);
			}

		   const code = new CodePromise(api, factory_metadata, factory_wasm);
		   // maximum gas to be consumed for the instantiation. if limit is too small the instantiation will fail.
			const gasLimit = 100000n * 1000000n
			// a limit to how much Balance to be used to pay for the storage created by the instantiation
			// if null is passed, unlimited balance can be used
			const storageDepositLimit = null
			// used to derive contract address,
			// use null to prevent duplicate contracts
			const initValue = dao_code_stored;

			const tx = code.tx.new({ gasLimit, storageDepositLimit }, initValue)

			let address;

			await tx.signAndSend(alice, ({ contract, status }) => {
			  if (status.isInBlock || status.isFinalized) {
				console.log('Included at block hash', status.asInBlock.toHex());
         		console.log('Events:');

         		events.forEach( ({ event: { data, method, section }, phase }) => {
				console.log('\t', phase.toString(), `: ${section}.${method}`, data.toString());
				})

			    address = contract.address.toString();
				  console.log("factory address: ", address)
				  process.exit(0)
			  }
			});

       }
     });


}

main().catch(console.error)
