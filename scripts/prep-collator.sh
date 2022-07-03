#!/bin/bash

# Prep bins for docker
function prepBins {
    cargo build --release
    find target/release -type f ! -name parachain-collator -exec rm {} +

    docker build --pull --rm -f ".github/dockerfiles/Dockerfile.collator" -t chocnet/parachain-collator "." &&
    gp sync-done collator
}

# Export chain spec, etc to ch_spec dir.
function exportChainSpec {
    mkdir ch_spec
    ./target/release/parachain-collator build-spec --disable-default-bootnode > ch_spec/rococo-local-parachain-plain.json
    ./target/release/parachain-collator build-spec --chain ch_spec/rococo-local-parachain-plain.json --raw --disable-default-bootnode > ch_spec/rococo-local-parachain-2000-raw.json
    ./target/release/parachain-collator export-genesis-wasm --chain ch_spec/rococo-local-parachain-2000-raw.json > ch_spec/para-2000-wasm
    ./target/release/parachain-collator export-genesis-state --chain ch_spec/rococo-local-parachain-2000-raw.json > ch_spec/para-2000-genesis
}

function main {
    prepBins
    exportChainSpec
}
"$@"