#!/usr/bin/env bash

set -e

dc="/usr/bin/parachain-collator"

if [ ! -x "$dc" ]; then
    echo "FATAL: no correct executables"
    exit 1
fi


get_id () {
    ./wait-for-it.sh "172.28.1.1:9933" -t 100 -- \
    curl -sS -H 'Content-Type: application/json' \
        --data '{"id":1,"jsonrpc":"2.0","method":"system_localPeerId"}' \
        "172.28.1.1:9933" | jq -r '.result'
}

echo -e "\n\nStarting collator..."
# Start collator
/usr/bin/parachain-collator \
--alice \
--collator \
--force-authoring \
--chain /chainspecs/rococo-local-parachain-2000-raw.json \
--base-path /data/.tmp/parachain/alice \
--port 30333 \
--ws-port 9944 \
--rpc-port 9933 \
--unsafe-rpc-external \
--unsafe-ws-external \
--no-prometheus \
--no-telemetry \
--rpc-cors=all \
-- \
--execution wasm \
--chain /chainspecs/rococo-local-raw.json \
--port 40333 \
--ws-port 7744 \
--rpc-port 7733 \
--bootnodes "/ip4/172.28.1.1/tcp/30333/p2p/$(get_id)"
--no-prometheus \
--no-telemetry
