#!/usr/bin/env bash
set -ex

cd "$(dirname "$0")"

cargo build-bpf
cp ./target/deploy/safecoin_sbf_rust_noop.so .
