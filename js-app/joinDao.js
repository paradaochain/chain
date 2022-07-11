#!/usr/bin/env node

const { Keyring } = require("@polkadot/keyring");
const { ApiPromise, WsProvider } = require("@polkadot/api");
const path = require("path");
const fs = require("fs");
const { ContractPromise } = require("@polkadot/api-contract");

// yarn joindao 5GtFrvFu9JRfipZZvBvw5XoVT8EFkbknccYjHvgJTRjNLSxQ Charlie 1000000

async function main() {
	const ip = "127.0.0.1";
	const port = "9944";

	const daoContractAddress = process.argv[2];
	const who = process.argv[3];
	const value = parseInt(process.argv[4]);

	// const daoContractAddress = fs.readFileSync(path.resolve( __dirname, ".tmp-daoContract"))
	const dao_metadata = fs
		.readFileSync(path.resolve(__dirname, "../target/ink/dao/metadata.json"))
		.toString();

	const wsProvider = new WsProvider(`ws://${ip}:${port}`);

	const api = await ApiPromise.create({
		provider: wsProvider,
	});

	const keyring = new Keyring({ type: "sr25519" });
	const alice = keyring.addFromUri("//" + who);

	const contract = new ContractPromise(api, dao_metadata, daoContractAddress);
	// maximum gas to be consumed for the call. if limit is too small the call will fail.
	const gasLimit = 1000000n * 1000000n;
	// a limit to how much Balance to be used to pay for the storage created by the contract call
	// if null is passed, unlimited balance can be used
	const storageDepositLimit = null;

	// (We perform the send from an account, here using Alice's address)

	await contract.tx
		.join({ storageDepositLimit, gasLimit, value }, "did:key:" + alice.address)
		.signAndSend(alice, (result) => {
			if (result.status.isInBlock) {
				result.events.forEach(({ event: { data, method, section }, phase }) => {
					console.log("\t", phase.toString(), `: ${section}.${method}`, data.toString());
				});
				console.log(JSON.stringify(result.contractEvents));
			} else if (result.status.isFinalized) {
				console.log("joined: ");
				process.exit(0);
			}
		});
}

main().catch(console.error);
