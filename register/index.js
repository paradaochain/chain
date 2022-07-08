#!/usr/bin/env node

const { Keyring } = require('@polkadot/keyring');
const { ApiPromise, WsProvider } = require("@polkadot/api");
const path = require("path")
const fs = require("fs")


async function main () {
  const ip = process.argv[2];
  const port = process.argv[3];
  const runtimePath = process.argv[4];
  const genesisState = process.argv[5];
  const expectedParaId = process.argv[6];

  const paradao_runtimeFile = fs.readFileSync(path.resolve(__dirname, runtimePath)).toString();
  const paradao_genesisFile = fs.readFileSync(path.resolve(__dirname, genesisState)).toString();

  const wsProvider = new WsProvider(`ws://${ip}:${port}`);

  const api = await ApiPromise.create({
    provider: wsProvider,
  });

  const keyring = new Keyring({ type: 'sr25519' });
  const alice = keyring.addFromUri('//Alice');
  let registeredId;

   await api.tx.registrar.reserve()
     .signAndSend(alice, async ({ events = [], status }) => {
		console.log('\n\n Reserve paraID: Transaction status:', status.type);

       if (status.isInBlock) {
         console.log('Included at block hash', status.asInBlock.toHex());
         console.log('Events:');

         events.forEach( ({ event: { data, method, section }, phase }) => {
           console.log('\t', phase.toString(), `: ${section}.${method}`, data.toString());
     		if (section == "registrar" && method == "Reserved"){
     			registeredId = data[0].toString();
				if (registeredId != expectedParaId) {
					console.log("\n\nPlease restart Relay Chain, expect " + expectedParaId + " but got: ", registeredId)
					process.exit(1)
				}
     		}
         });
       } else if (status.isFinalized) {
            console.log('Finalized block hash', status.asFinalized.toHex());
		    console.log("Genesis File: ", genesisFile);
			await api.tx.sudo.sudo(api.tx.
				parasSudoWrapper.sudoScheduleParaInitialize(registeredId, {
				"genesisHead": genesisFile,
				"validationCode": runtimeFile,
				"parachain": true
			}))
    		.signAndSend(alice, ({ events = [], status }) => {
				console.log('\n\n Register Parachain: Transaction status:', status.type);

    		  if (status.isInBlock) {
    		    console.log('Included at block hash', status.asInBlock.toHex());
    		    console.log('Events:');

    		    events.forEach(({ event: { data, method, section }, phase }) => {
    		      console.log('\t', phase.toString(), `: ${section}.${method}`, data.toString());
    		    });
    		  } else if (status.isFinalized) {
    		    console.log('Finalized block hash', status.asFinalized.toHex());
				process.exit(0)

    		  }
    		});
       }
     });


}

main().catch(console.error)
