#!/bin/bash

# Run node1
./target/release/parachain-collator \
--alice \
--collator \
--force-authoring \
--chain ch_spec/rococo-local-parachain-2000-raw.json \
--base-path /tmp/parachain/alice \
--port 40333 \
--ws-port 8844 \
-- \
--execution wasm \
--chain ~/relay/polkadot/rococo-local-raw.json \
--port 30343 \
--ws-port 9977
