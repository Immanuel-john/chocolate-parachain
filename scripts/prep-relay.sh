#!/bin/bash

# Zombienet: Build relay image
function makeBins {
    mkdir bins
    cp ~/relay/polkadot/target/release/polkadot bins/
    docker build --pull --rm -f ".github/dockerfiles/Dockerfile.relay" -t chocnet/polkadot-debug "."
    rm -r bins
    gp sync-done relay
}
# Setup files for local
function buildSpec {
    cd ~/relay/polkadot

    ./target/release/polkadot build-spec  --chain rococo-local --disable-default-bootnode > ./rococo-local.json
    ./target/release/polkadot build-spec  --chain ./rococo-local.json --disable-default-bootnode --raw > ./rococo-local-raw.json

}

function main {
    makeBins
    buildSpec
}

"$@"