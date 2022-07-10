#!/usr/bin/env node

const { Keyring } = require('@polkadot/keyring');
const { ApiPromise, WsProvider } = require("@polkadot/api");
const path = require("path")
const fs = require("fs")
const { ContractPromise } = require('@polkadot/api-contract');

// yarn propose Charlie "To fund social media" ipfshash 5GNJqTPyNqANBkUVMN1LPPrxXnFouWXoe2wNSmmEoLctxiZY 1000 5GtFrvFu9JRfipZZvBvw5XoVT8EFkbknccYjHvgJTRjNLSxQ


async function main () {

  const ip = "127.0.0.1"
  const port = "9944"

  const who = process.argv[2]
  const title = process.argv[3]
  const metadataUrl = process.argv[4]
  const to = process.argv[5]
  const balance = process.argv[6]
  const daoContractAddress = process.argv[7]

  // const daoContractAddress = fs.readFileSync(path.resolve( __dirname, ".tmp-daoContract")).toString('utf-8');
  console.log("daocontract: ", daoContractAddress);
  const dao_metadata= fs.readFileSync(path.resolve(__dirname,
		"../target/ink/dao/metadata.json")).toString();

  const wsProvider = new WsProvider(`ws://${ip}:${port}`);

  const api = await ApiPromise.create({
    provider: wsProvider,
	  types: {
		ProposalType: {
			_enum: {
        	  Treasury: 'Treasury',
        	  Membership: 'Membership',
        	  Proxy: 'Proxy',
        	  UpdateMetadata: 'String',
        	  UpdateFee: 'u128'
        	}
		},
		Treasury: '(AccountId, Balance)',
		Membership: '(Vec<String>, Vec<(String, Role)>)',
		Proxy: 'Transaction'
	 }
  });

  const keyring = new Keyring({ type: 'sr25519' });
  const alice = keyring.addFromUri('//' + who);

  const contract = new ContractPromise(api, dao_metadata, daoContractAddress);
  const gasLimit = 1000000n * 1000000n;
  const storageDepositLimit = null


	let proposal = {"Treasury": [to, balance] };

	 await contract.tx
		.propose({ storageDepositLimit, gasLimit  }, proposal, title, metadataUrl)
	   .signAndSend(alice, result => {
	     if (result.status.isInBlock) {
           result.events.forEach( ({ event: { data, method, section }, phase }) => {
             console.log('\t', phase.toString(), `: ${section}.${method}`, data.toString());
           });
			console.log(JSON.stringify(result.contractEvents))
	     } else if (result.status.isFinalized) {
			 console.log('proposed: ' );
			 process.exit(0)
	     }
	   });


}

main().catch(console.error)
