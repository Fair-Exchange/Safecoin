safecoin_sdk::declare_builtin!(
    safecoin_sdk::bpf_loader_upgradeable::ID,
    solana_bpf_loader_upgradeable_program,
    solana_bpf_loader_program::process_instruction,
    upgradeable::id
);
