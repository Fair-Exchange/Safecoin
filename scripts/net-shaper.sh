#!/usr/bin/env bash
#
# Start/Stop network shaper
#
set -e

[[ $(uname) == Linux ]] || exit 0

cd "$(dirname "$0")"

sudo=
if sudo true; then
  sudo="sudo -n"
fi

set -x

iface="$(ip link show | grep mtu | grep -iv loopback | grep "state UP" | awk 'BEGIN { FS = ": " } ; {print $2}')"

if [[ "$1" = cleanup ]]; then
  $sudo ~solana/.cargo/bin/safecoin-net-shaper cleanup -f "$2" -s "$3" -p "$4" -i "$iface"
else
  $sudo ~solana/.cargo/bin/safecoin-net-shaper shape -f "$2" -s "$3" -p "$4" -i "$iface"
fi
