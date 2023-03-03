# source this file

update_solana_dependencies() {
  declare project_root="$1"
  declare solana_ver="$2"
  declare tomls=()
  while IFS='' read -r line; do tomls+=("$line"); done < <(find "$project_root" -name Cargo.toml)

  sed -i -e "s#\(safecoin-program = \"\)[^\"]*\(\"\)#\1=$solana_ver\2#g" "${tomls[@]}" || return $?
  sed -i -e "s#\(safecoin-program-test = \"\)[^\"]*\(\"\)#\1=$solana_ver\2#g" "${tomls[@]}" || return $?
  sed -i -e "s#\(safecoin-sdk = \"\).*\(\"\)#\1=$solana_ver\2#g" "${tomls[@]}" || return $?
  sed -i -e "s#\(safecoin-sdk = { version = \"\)[^\"]*\(\"\)#\1=$solana_ver\2#g" "${tomls[@]}" || return $?
  sed -i -e "s#\(safecoin-client = \"\)[^\"]*\(\"\)#\1=$solana_ver\2#g" "${tomls[@]}" || return $?
  sed -i -e "s#\(safecoin-client = { version = \"\)[^\"]*\(\"\)#\1=$solana_ver\2#g" "${tomls[@]}" || return $?
  sed -i -e "s#\(safecoin-clap-utils = \"\)[^\"]*\(\"\)#\1=$solana_ver\2#g" "${tomls[@]}" || return $?
  sed -i -e "s#\(safecoin-clap-utils = { version = \"\)[^\"]*\(\"\)#\1=$solana_ver\2#g" "${tomls[@]}" || return $?
  sed -i -e "s#\(safecoin-account-decoder = \"\)[^\"]*\(\"\)#\1=$solana_ver\2#g" "${tomls[@]}" || return $?
  sed -i -e "s#\(safecoin-account-decoder = { version = \"\)[^\"]*\(\"\)#\1=$solana_ver\2#g" "${tomls[@]}" || return $?
  sed -i -e "s#\(safecoin-faucet = \"\)[^\"]*\(\"\)#\1=$solana_ver\2#g" "${tomls[@]}" || return $?
  sed -i -e "s#\(safecoin-faucet = { version = \"\)[^\"]*\(\"\)#\1=$solana_ver\2#g" "${tomls[@]}" || return $?
  sed -i -e "s#\(safe-zk-token-sdk = \"\)[^\"]*\(\"\)#\1=$solana_ver\2#g" "${tomls[@]}" || return $?
  sed -i -e "s#\(safe-zk-token-sdk = { version = \"\)[^\"]*\(\"\)#\1=$solana_ver\2#g" "${tomls[@]}" || return $?
}

patch_crates_io_solana() {
  declare Cargo_toml="$1"
  declare solana_dir="$2"
  cat >> "$Cargo_toml" <<EOF
[patch.crates-io]
safecoin-account-decoder = { path = "$solana_dir/account-decoder" }
safecoin-clap-utils = { path = "$solana_dir/clap-utils" }
safecoin-client = { path = "$solana_dir/client" }
safecoin-program = { path = "$solana_dir/sdk/program" }
safecoin-program-test = { path = "$solana_dir/program-test" }
safecoin-sdk = { path = "$solana_dir/sdk" }
safecoin-faucet = { path = "$solana_dir/faucet" }
safe-zk-token-sdk = { path = "$solana_dir/zk-token-sdk" }
EOF
}
