### Chain spec break down

Literally just comments trying to understand pieces of the chainspec.

```jsonc

        ...
        "paras": {
            // array of registered parachains, with ids. Must be correctly assigned
          "paras": []
        },
        ...
        ...
       "sudo": {
        // Sudo, alice
          "key": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"
        },
        ...
    

```

### utils

`$RELAY_BIN key inspect ${key derivation path or key}`, inspect keys.
e.g:
```console
gitpod ~/relay/polkadot (release-v0.9.23) $ ./target/release/polkadot key inspect //Alce
Secret Key URI `//Alce` is account:
  Network ID:        substrate 
 Secret seed:       0x2b853e582682f6d9f842ddbb8d03e58de96f33d9276e051c492463addfd8c37f
  Public key (hex):  0x2c12a8387f0a9aa821cc520fe3cd7a5b9b74e0c403a40188fd10f5887112a616
  Account ID:        0x2c12a8387f0a9aa821cc520fe3cd7a5b9b74e0c403a40188fd10f5887112a616
  Public key (SS58): 5D4VWKoog7oFQrSVWRWisYyyDHs1p2g4DNhU84NWRBLLDme3
  SS58 Address:      5D4VWKoog7oFQrSVWRWisYyyDHs1p2g4DNhU84NWRBLLDme3
```

### Generate chain spec:

./target/release/polkadot build-spec  --chain rococo-local  > ./rococo-local.json
./target/release/polkadot build-spec  --chain rococo-local  --raw > ./rococo-local-raw.json

### Alice

```
./target/release/polkadot \
--alice \
--validator \
--base-path /tmp/relay/alice \
--chain ./rococo-local-raw.json \
--port 30333 \
--ws-port 9944

```
12D3KooWF2oFXQNxTdpRqKKhcUgbPzhNusTXWGTmLFfRqpcviyk5

### Bob


./target/release/polkadot \
--bob \
--validator \
--base-path /tmp/relay-bob \
--chain ./rococo-local-raw.json \
--bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWF2oFXQNxTdpRqKKhcUgbPzhNusTXWGTmLFfRqpcviyk5 \
--port 30334 \
--ws-port 9945


### Init:
```
./target/release/polkadot --alice --validator --base-path /tmp/relay/alice --chain ./rococo-local-raw.json --port 30333 --ws-port 9944
<!-- Bob -->
./target/release/polkadot \
--bob \
--validator \
--base-path /tmp/relay-bob \
--chain ./rococo-local-raw.json \
--bootnodes /ip4/127.0.0.1/tcp/30333/p2p/Alice-node-addr \
--port 30334 \
--ws-port 9945
```