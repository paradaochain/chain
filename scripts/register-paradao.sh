# #!/usr/bin/env bash
#
# set -e
#
# temp="./.tmp"
#
# if [ ! -x "$temp" ]; then
#     mkdir ./.tmp
# fi
#
#
# # Generate wasm
# echo -e "\n\nGenerating Parachain Runtime WASM"
# ./target/release/parachain-collator export-genesis-wasm \
#     --chain ./chainspecs/rococo-local-parachain-2000-raw.json > ./.tmp/para-2000-wasm
#
# # Generate genesis state
# echo -e "\n\nGenerating Parachain Genesis state"
# ./target/release/parachain-collator export-genesis-state \
#     --chain ./chainspecs/rococo-local-parachain-2000-raw.json > ./.tmp/para-2000-genesis
#
# echo -e "\n\nRunning NodeJS app to reserve and register parachain slot"
# cd ./register
# yarn install
# cd ..
# node register 127.0.0.1 6644 \
#        ../.tmp/para-2000-wasm \
#        ../.tmp/para-2000-genesis


get_id () {
    curl -sS -H 'Content-Type: application/json' \
        --data '{"id":1,"jsonrpc":"2.0","method":"system_localPeerId"}' \
        "127.0.0.1:6633" | jq -r '.result'
}

echo -e "\n\nStarting collator..."
# Start collator
./target/release/parachain-collator \
--alice \
--collator \
--force-authoring \
--chain ./chainspecs/rococo-local-parachain-2000-raw.json \
--base-path .tmp/parachain/alice \
--port 20333 \
--ws-port 8844 \
--rpc-port 8833 \
--rpc-cors=all \
--unsafe-rpc-external \
--unsafe-ws-external \
-- \
--execution wasm \
--chain ./chainspecs/rococo-local-raw.json \
--port 40333 \
--ws-port 7744 \
--rpc-port 7733 \
--bootnodes "/ip4/127.0.0.1/tcp/30333/p2p/$(get_id)"
