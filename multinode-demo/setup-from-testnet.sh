#!/usr/bin/env bash

here=$(dirname "$0")
# shellcheck source=multinode-demo/common.sh
source "$here"/common.sh

set -e

rm -rf "$SAFEANA_CONFIG_DIR"/latest-testnet-snapshot
mkdir -p "$SAFEANA_CONFIG_DIR"/latest-testnet-snapshot
(
  cd "$SAFEANA_CONFIG_DIR"/latest-testnet-snapshot || exit 1
  set -x
  wget http://api.testnet.safecoin.org/genesis.tar.bz2
  wget --trust-server-names http://testnet.safecoin.org/snapshot.tar.bz2
)

snapshot=$(ls "$SAFEANA_CONFIG_DIR"/latest-testnet-snapshot/snapshot-[0-9]*-*.tar.zst)
if [[ -z $snapshot ]]; then
  echo Error: Unable to find latest snapshot
  exit 1
fi

if [[ ! $snapshot =~ snapshot-([0-9]*)-.*.tar.zst ]]; then
  echo Error: Unable to determine snapshot slot for "$snapshot"
  exit 1
fi

snapshot_slot="${BASH_REMATCH[1]}"

rm -rf "$SAFEANA_CONFIG_DIR"/bootstrap-validator
mkdir -p "$SAFEANA_CONFIG_DIR"/bootstrap-validator


# Create genesis ledger
if [[ -r $FAUCET_KEYPAIR ]]; then
  cp -f "$FAUCET_KEYPAIR" "$SAFEANA_CONFIG_DIR"/faucet.json
else
  $safecoin_keygen new --no-passphrase -fso "$SAFEANA_CONFIG_DIR"/faucet.json
fi

if [[ -f $BOOTSTRAP_VALIDATOR_IDENTITY_KEYPAIR ]]; then
  cp -f "$BOOTSTRAP_VALIDATOR_IDENTITY_KEYPAIR" "$SAFEANA_CONFIG_DIR"/bootstrap-validator/identity.json
else
  $safecoin_keygen new --no-passphrase -so "$SAFEANA_CONFIG_DIR"/bootstrap-validator/identity.json
fi

$safecoin_keygen new --no-passphrase -so "$SAFEANA_CONFIG_DIR"/bootstrap-validator/vote-account.json
$safecoin_keygen new --no-passphrase -so "$SAFEANA_CONFIG_DIR"/bootstrap-validator/stake-account.json

$safecoin_ledger_tool create-snapshot \
  --ledger "$SAFEANA_CONFIG_DIR"/latest-testnet-snapshot \
  --faucet-pubkey "$SAFEANA_CONFIG_DIR"/faucet.json \
  --faucet-lamports 1000 \
  --bootstrap-validator "$SAFEANA_CONFIG_DIR"/bootstrap-validator/identity.json \
                        "$SAFEANA_CONFIG_DIR"/bootstrap-validator/vote-account.json \
                        "$SAFEANA_CONFIG_DIR"/bootstrap-validator/stake-account.json \
  --hashes-per-tick sleep \
  "$snapshot_slot" "$SAFEANA_CONFIG_DIR"/bootstrap-validator

$safecoin_ledger_tool modify-genesis \
  --ledger "$SAFEANA_CONFIG_DIR"/latest-testnet-snapshot \
  --hashes-per-tick sleep \
  "$SAFEANA_CONFIG_DIR"/bootstrap-validator
