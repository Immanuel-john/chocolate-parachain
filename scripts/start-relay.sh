#!/bin/bash

cd ~/relay/polkadot
# echo start alice command, 
echo "Start Alice command"
echo "bash ./scripts/start-relay.sh alice"
echo ""

function alice {
    cd ~/relay/polkadot
    ./target/release/polkadot --alice --validator --base-path /tmp/relay/alice --chain ./rococo-local-raw.json --port 30333 --ws-port 9944
}
echo "Start Bob command"
echo "bash ./scripts/start-relay.sh bob --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/AliceAddr "
echo ""

function bob {
    cd ~/relay/polkadot

    ./target/release/polkadot \
    --bob \
    --validator \
    --base-path /tmp/relay/bob \
    --chain ./rococo-local-raw.json \
    --port 30334 \
    --ws-port 9945 \
    --bootnodes "$1"
}

"$@"