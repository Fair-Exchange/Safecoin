#!/usr/bin/env bash
set -ex

cd "$(dirname "$0")"

# shellcheck source=net/scripts/solana-user-authorized_keys.sh
source solana-user-authorized_keys.sh

# solana-user-authorized_keys.sh defines the public keys for users that should
# automatically be granted access to ALL datacenter nodes.
for i in "${!SAFEANA_USERS[@]}"; do
  echo "environment=\"SAFEANA_USER=${SAFEANA_USERS[i]}\" ${SAFEANA_PUBKEYS[i]}"
done

