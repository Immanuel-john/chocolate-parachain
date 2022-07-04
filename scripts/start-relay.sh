#!/bin/bash

# echo start alice command, 
echo "Start Alice command"
echo "bash ./scripts/start-relay.sh alice"
echo ""

RELAY_BIN=~/relay/polkadot/target/release/polkadot
ROCOCO_LOCAL=ch_spec/rococo-local.json
ROCOCO_LOCAL_RAW=ch_spec/rococo-local-raw.json

function alice {
    $RELAY_BIN --alice --validator --base-path /tmp/relay/alice --chain $ROCOCO_LOCAL_RAW --port 30333 --ws-port 9944
}
echo "Start Bob command"
echo "bash ./scripts/start-relay.sh bob"
echo ""

function bob {

    $RELAY_BIN \
    --bob \
    --validator \
    --base-path /tmp/relay/bob \
    --chain $ROCOCO_LOCAL_RAW \
    --port 30334 \
    --ws-port 9945 \
    #  Mdns works
}

"$@"