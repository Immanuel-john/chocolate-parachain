#!/bin/bash

# Zombienet: Build relay image
function makeBins {
    mkdir bins
    cp $POLKADOT_BIN bins/
    docker build --pull --rm -f ".github/dockerfiles/Dockerfile.relay" -t chocnet/polkadot-debug "."
}
# Setup files for local
function buildSpec {
    mkdir -p ch_spec
    RELAY_BIN=./bins/polkadot

    $RELAY_BIN build-spec  --chain rococo-local --disable-default-bootnode > ch_spec/rococo-local.json
    $RELAY_BIN build-spec  --chain ch_spec/rococo-local.json --disable-default-bootnode --raw > ch_spec/rococo-local-raw.json
    gp sync-done relay
}

function main {
    makeBins
    buildSpec
}

"$@"