#!/usr/bin/env bash
#
# Fetches the latest SPL programs and produces the safecoin-genesis command-line
# arguments needed to install them
#

set -e

fetch_program() {
  declare name=$1
  declare version=$2
  declare address=$3
  declare loader=$4

  declare so=spl_$name-$version.so

  genesis_args+=(--bpf-program "$address" "$loader" "$so")

  if [[ -r $so ]]; then
    return
  fi

  if [[ -r ~/.cache/solana-spl/$so ]]; then
    cp ~/.cache/solana-spl/"$so" "$so"
  else
    echo "Downloading $name $version"
    so_name="spl_${name//-/_}.so"
    (
      set -x
      curl -L --retry 5 --retry-delay 2 --retry-connrefused \
        -o "$so" \
        "https://github.com/fair-exchange/safecoin-program-library/releases/download/$name-v$version/$so_name"
    )

    mkdir -p ~/.cache/solana-spl
    cp "$so" ~/.cache/solana-spl/"$so"
  fi

}

fetch_program token 3.1.0 7v5TwK92hUSqduoL3R8NtzTNfNzMA48nJL4mzPYMdDrD BPFLoader2111111111111111111111111111111111
fetch_program memo  1.0.0 4DDUJ1rA8Vd7e6SFWanf4V8JnsfapjCGNutQYw8Vtt45 BPFLoader1111111111111111111111111111111111
fetch_program memo  3.0.0 9h7wfE8nxQ6YsRedqNHwroEZbA5bMAmNsh8GdxwBTtaV BPFLoader2111111111111111111111111111111111
fetch_program associated-token-account 1.0.1 CWyEp7dp1Cv3334j6gCci2UrrjA8Q98bYa7AwGBpZ6iJ BPFLoader2111111111111111111111111111111111
fetch_program feature-proposal 1.0.0 D5WhRrnh8AefVULhudnuLA1LCVzvEgdn49eDWE67wYNn BPFLoader2111111111111111111111111111111111

echo "${genesis_args[@]}" > spl-genesis-args.sh

echo
echo "Available SPL programs:"
ls -l spl_*.so

echo
echo "safecoin-genesis command-line arguments (spl-genesis-args.sh):"
cat spl-genesis-args.sh
