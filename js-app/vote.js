#!/usr/bin/env node

const { Keyring } = require("@polkadot/keyring");
const { ApiPromise, WsProvider } = require("@polkadot/api");
const path = require("path");
const fs = require("fs");
const { ContractPromise } = require("@polkadot/api-contract");

// yarn vote Charlie 0 true 5GtFrvFu9JRfipZZvBvw5XoVT8EFkbknccYjHvgJTRjNLSxQ

async function main() {
	const ip = "127.0.0.1";
	const port = "9944";

	const who = process.argv[2];
	const proposalId = process.argv[3];
	const vote = process.argv[4];
	const daoContractAddress = process.argv[5];

	// const daoContractAddress = fs.readFileSync(path.resolve( __dirname, ".tmp-daoContract")).toString('utf-8');
	console.log("daocontract: ", daoContractAddress);
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
	const gasLimit = 1000000n * 1000000n;
	const storageDepositLimit = null;
	let status = null;

	await contract.tx
		.vote({ storageDepositLimit, gasLimit }, proposalId, vote)
		.signAndSend(alice, async (result) => {
			if (result.status.isInBlock) {
				result.events.forEach(({ event: { data, method, section }, phase }) => {
					console.log("\t", phase.toString(), `: ${section}.${method}`, data.toString());
				});
				console.log("contractEvents", result.contractEvents)
				if (result.contractEvents) {
					console.log(JSON.stringify(result.contractEvents[0]["args"]));
					status = result.contractEvents[0]["args"][3].isPassed
				}
			} else if (result.status.isFinalized) {
				console.log("finalised vote");
				console.log("Status to execute proposal ", status);
				if (status == true) {
					await contract.tx
						.execute({ storageDepositLimit, gasLimit }, proposalId)
						.signAndSend(alice, (result) => {
							if (result.status.isInBlock) {
								result.events.forEach(
									({ event: { data, method, section }, phase }) => {
										console.log(
											"\t",
											phase.toString(),
											`: ${section}.${method}`,
											data.toString()
										);
									}
								);
								if (result.contractEvents) {
									console.log(
										"Executed proposal: ",
										result.contractEvents[0]["args"][0]
									);
								}
							}else if (result.status.isFinalized) {
								console.log("finalised execute");
								process.exit(0)

							}

						});
				}
				process.exit(0);
			}
		});
}

main().catch(console.error);
