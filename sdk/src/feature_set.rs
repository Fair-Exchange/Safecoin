use {
    lazy_static::lazy_static,
    safecoin_sdk::{
        clock::Slot,
        hash::{Hash, Hasher},
        pubkey::Pubkey,
    },
    std::collections::{HashMap, HashSet},
};

pub mod instructions_sysvar_enabled {
    safecoin_sdk::declare_id!("7TfFp6Tf2XqXQQfx16qvXbjekXtn68kiQj9pPfXZ5Bua");
}

pub mod consistent_recent_blockhashes_sysvar {
    safecoin_sdk::declare_id!("ApdySrEmykK3PBLExdN2ehGCRPyVT6athFgu4H7H8e9J");
}

pub mod deprecate_rewards_sysvar {
    safecoin_sdk::declare_id!("GVriKcZATdgSzmhmu9PtimfheaKYksK5mhgoN5fbRhXg");
}

pub mod pico_inflation {
    safecoin_sdk::declare_id!("HC5H1tGb2RBRbrECSKmBVjwTsEEJziTTRsm3XSdTefrJ");
}

pub mod full_inflation {
    pub mod devnet_and_testnet {
        safecoin_sdk::declare_id!("JC1GQujpgHz92Am65FxcxByqqDGTdYJs6jLjgebnDpJF");
    }

    pub mod mainnet {
        pub mod certusone {
            pub mod vote {
                safecoin_sdk::declare_id!("7Ra6gKzEBFyYJ7fdsrCuzihZcxKu7AbmDy2FBEQFzap2");
            }
            pub mod enable {
                safecoin_sdk::declare_id!("54ZqXvMyCKShnBrGj2ni6m4kXTjRUrkc21dFoT9npJZk");
            }
        }
    }
}

pub mod safe_token_v2_multisig_fix {
    safecoin_sdk::declare_id!("EwSWSBRpzYZSEqdZ215WMmnce6WiEsk57rSEB3e7ghh6");
}

pub mod no_overflow_rent_distribution {
    safecoin_sdk::declare_id!("9TyDRDhs933rTCWGwzSUTSx1XeJT14sc17o1cNQzUaBq");
}

pub mod filter_stake_delegation_accounts {
    safecoin_sdk::declare_id!("HpGqShCRhP7QwMBXTs1KbATiHWa383EUWjg3kbQjN2Kf");
}

pub mod bpf_loader_upgradeable_program {
    safecoin_sdk::declare_id!("Cv6gGxiakDF6nd9Sxx53MbC4kij69qXpap8guiC6aK9U");
}

pub mod stake_program_v3 {
    safecoin_sdk::declare_id!("6tYrCsaWbGqgeW9tN3NRbViw6BBLYjnNsJBqLJZZoo5B");
}

pub mod require_custodian_for_locked_stake_authorize {
    safecoin_sdk::declare_id!("FKWSvfcXATHSBBNvr5VE6ns4tNsTG3EGzcDw2xVtowZQ");
}

pub mod safe_token_v2_self_transfer_fix {
    safecoin_sdk::declare_id!("2XDc17ZmSTbpqV3B5fmEGac4CKCYnbJj7vnfASvdzqyN");
}

pub mod warp_timestamp_again {
    safecoin_sdk::declare_id!("EgfbGcUzJotpXW8H4TnhVwDbrjm75xecKnrSAfqv4YvW");
}

pub mod check_init_vote_data {
    safecoin_sdk::declare_id!("CfH5RJxvjkQ1LaBSrnCDnYvHZtL6FuTQ751myi6DJEb5");
}

pub mod check_program_owner {
    safecoin_sdk::declare_id!("93ksjAw2xaVgWcBaqQSasj6hNiX3yUkPLpVZ8hoE2jE6");
}

pub mod require_stake_for_gossip {
    safecoin_sdk::declare_id!("7WM5jgvcKh9yyEj4NhG6qt1MvahRB6mhkodaY5KQwT4k");
}

pub mod cpi_data_cost {
    safecoin_sdk::declare_id!("6U62sD7ffNehUa7QLia5DUPNmFvGeZgrGLnRPo5Jdw9v");
}

pub mod upgradeable_close_instruction {
    safecoin_sdk::declare_id!("EzdQuAfpfg1pDTLyj52PLMPJDDCo4aqzncsSprFvMvU5");
}

pub mod sysvar_via_syscall {
    safecoin_sdk::declare_id!("92ep2YChAdo4XiDPmNkRcAfJZuRv46n5ko9z6S512awn");
}

pub mod enforce_aligned_host_addrs {
    safecoin_sdk::declare_id!("6yrRvLK2AT8U9dq4aSArH3MSRdHbSYQsCd99zHxeAXJ8");
}

pub mod set_upgrade_authority_via_cpi_enabled {
    safecoin_sdk::declare_id!("8voJR1noBKPVBgE7BV6g3DCu4FJshGmCs1dRPuKnyJH5");
}

pub mod update_data_on_realloc {
    safecoin_sdk::declare_id!("DcTaQRy9wRBGywHkFStEnBW8V7yiJkGADXYbByjM3UEQ");
}

pub mod keccak256_syscall_enabled {
    safecoin_sdk::declare_id!("8ubpzsyCpCcDVccAnPNuQV439SPw5xdCpFryBMU8wPv1");
}

pub mod stake_program_v4 {
    safecoin_sdk::declare_id!("8Sgh17Pmrt6kKCJuNFGzJTnwRSMt1ELCSBuYThLznCYR");
}

pub mod memory_ops_syscalls {
    safecoin_sdk::declare_id!("45aF3i2bmR2e9bPnZNpyRVZexjbaYKyq7f4PG59jhXxg");
}

pub mod secp256k1_recover_syscall_enabled {
    safecoin_sdk::declare_id!("6RvdSWHh8oh72Dp7wMTS2DBkf3fRPtChfNrAo3cZZoXJ");
}

pub mod add_missing_program_error_mappings {
    safecoin_sdk::declare_id!("3QEUpjhgPEt92nz3Mqf6pABkHPGCQwSvKtyGMq4SuQyL");
}

pub mod system_transfer_zero_check {
    safecoin_sdk::declare_id!("ACrERgP7eXsQquYUNQwASR884izTTXnR2jDnxjWRPEde");
}

pub mod dedupe_config_program_signers {
    safecoin_sdk::declare_id!("Ffpg1Q73ocztQKMTbkTMeTAVpcGUb95u2aUaCeFQ47NY");
}

pub mod deterministic_shred_seed_enabled {
    safecoin_sdk::declare_id!("FjSRMpFe7mofQ3WrEMT7Smjk2sME1XdAoRxcv55V6M44");
}

pub mod verify_tx_signatures_len {
    safecoin_sdk::declare_id!("EVW9B5xD9FFK7vw1SBARwMA4s5eRo5eKJdKpsBikzKBz");
}

pub mod vote_stake_checked_instructions {
    safecoin_sdk::declare_id!("BjfQ729TvPN84xsvsGMUsRLAqGc1ttSCQ6C5Zd3rUg2N");
}

pub mod updated_verify_policy {
    safecoin_sdk::declare_id!("8JWHY3gNMHMMia4smn5fa6KH91znav6qUzLMAJVNaNoY");
}

pub mod neon_evm_compute_budget {
    safecoin_sdk::declare_id!("GLrVvDPkQi5PMYUrsYWT9doZhSHr1BVZXqj5DbFps3rS");
}

pub mod rent_for_sysvars {
    safecoin_sdk::declare_id!("BKCPBQQBZqggVnFso5nQ8rQ4RwwogYwjuUt9biBjxwNF");
}

pub mod libsecp256k1_0_5_upgrade_enabled {
    safecoin_sdk::declare_id!("EVHL8iX15Gf6PpjdMd7pqPivjPQ2LLbK9fkGsrsGWy9r");
}

pub mod stop_verify_mul64_imm_nonzero {
    safecoin_sdk::declare_id!("EHFwHg2vhwUb7ifm7BuY9RMbsyt1rS1rUii7yeDJtGnN");
}

pub mod start_verify_shift32_imm {
    safecoin_sdk::declare_id!("CqvdhqAYMc6Eq6tjW3H42Qg39TK2SCsL8ydMsC363PRp");
}

pub mod merge_nonce_error_into_system_error {
    safecoin_sdk::declare_id!("4n5Ko6ax8yLi21CXoBMFbCy52QydH7jpy42W5df7GZqT");
}

pub mod safe_token_v2_set_authority_fix {
    safecoin_sdk::declare_id!("Cb3jN13cfCNDV9dp36djNpcZXF7r82UAE4U1tZjXnFx5");
}

pub mod stake_merge_with_unmatched_credits_observed {
    safecoin_sdk::declare_id!("BrbirQE6i85NmJ3eB6jRLJbuYMT26HhSeHFYdxrfbUkk");
}

pub mod gate_large_block {
    safecoin_sdk::declare_id!("8oBgb4GCzTaLXjHNZfzQimCVS74vC6YcFGUz3Q91cwxz");
}

pub mod mem_overlap_fix {
    safecoin_sdk::declare_id!("B7hPRGNWY1uLvHmZPeZJKqes4jXjra9qwFQUojg9Fz8P");
}

pub mod close_upgradeable_program_accounts {
    safecoin_sdk::declare_id!("3DaizaRfafPaLYiaFjZbwyYv7MyVkLkHVoHft2TdxeeP");
}

pub mod stake_program_advance_activating_credits_observed {
    safecoin_sdk::declare_id!("8SgjbhRrT9Lu5gX8zyWtbeMynSMLfKs1VkDoVVbojSDp");
}

pub mod demote_program_write_locks {
    safecoin_sdk::declare_id!("3E3jV7v9VcdJL8iYZUMax9DiDno8j7EWUVbhm9RtShj2");
}

pub mod allow_native_ids {
    safecoin_sdk::declare_id!("3o9nmPV66aeNrKGM9NVwDngPmexxBjoVRntQb5EVCKN9");
}

pub mod check_seed_length {
    safecoin_sdk::declare_id!("4apMS2FxQTYL8d1xdqQY7pvwKjoSQKzVUAtJs1xhgMDi");
}

pub mod fix_write_privs {
    safecoin_sdk::declare_id!("GGYVzKJcvUaiTzFUF9bt9WECv2a7iRsSLgBzvQeeH7db");
}

pub mod reduce_required_deploy_balance {
    safecoin_sdk::declare_id!("CChcJjMA6MbJnEa3JN5vSuuitmiZCrpiJCuYpwLNc6xJ");
}

pub mod stakes_remove_delegation_if_inactive {
    safecoin_sdk::declare_id!("7cRMwESG7sdDrPBKWUWJZnUzPB1FRXj2YDCFRepjpcvX");
}

pub mod send_to_tpu_vote_port {
    safecoin_sdk::declare_id!("JAB9JKLm1zmdv9ntxnbTUxgGK9p2WdmDjzwf5ptMZQ4L");
}

pub mod optimize_epoch_boundary_updates {
    safecoin_sdk::declare_id!("9aRcTW6CA2gm66oFyRqp9bV6qF5VYHiuSxdxTGm78rEi");
}

pub mod tx_wide_compute_cap {
    safecoin_sdk::declare_id!("5ekBxc8itEnPv4NzGJtr8BVVQLNMQuLMNQQj7pHoLNZ9");
}


pub mod remove_native_loader {
    safecoin_sdk::declare_id!("ATWHHk7mdfaatNZTuKPUMvcBc9iKZStmph9ASzaWoSSj");
}

pub mod return_data_syscall_enabled {
    safecoin_sdk::declare_id!("DwScAzPUjuv65TMbDnFY7AgwmotzWy3xpEJMXM3hZFaB");
}

pub mod sol_log_data_syscall_enabled {
    safecoin_sdk::declare_id!("6uaHcKPGUy4J7emLBgUTeufhJdiwhngW6a1R9B7c2ob9");
}

pub mod ed25519_program_enabled {
    safecoin_sdk::declare_id!("6ppMXNYLhVd7GcsZ5uV11wQEW7spppiMVfqQv5SXhDpX");
}

pub mod requestable_heap_size {
    safecoin_sdk::declare_id!("CCu4boMmfLuqcmfTLPHQiUo22ZdUsXjgzPAURYaWt1Bw");
}

pub mod add_compute_budget_program {
    safecoin_sdk::declare_id!("4d5AKtxoh93Dwm1vHXUU3iRATuMndx1c431KgT2td52r");
}

pub mod reject_deployment_of_unresolved_syscalls {
    safecoin_sdk::declare_id!("DqniU3MfvdpU3yhmNF1RKeaM5TZQELZuyFGosASRVUoy");
}

pub mod reject_section_virtual_address_file_offset_mismatch {
    safecoin_sdk::declare_id!("5N4NikcJLEiZNqwndhNyvZw15LvFXp1oF7AJQTNTZY5k");
}

pub mod reject_all_elf_rw {
    safecoin_sdk::declare_id!("DeMpxgMq51j3rZfNK2hQKZyXknQvqevPSFPJFNTbXxsS");
}

pub mod safe_token_v3_3_0_release {
    safecoin_sdk::declare_id!("Ftok2jhqAqxUWEiCVRrfRs9DPppWP8cgTB7NQNKL88mS");
}

pub mod reject_non_rent_exempt_vote_withdraws {
    safecoin_sdk::declare_id!("7txXZZD6Um59YoLMF7XUNimbMjsqsWhc7g2EniiTrmp1");
}

pub mod evict_invalid_stakes_cache_entries {
    safecoin_sdk::declare_id!("EMX9Q7TVFAmQ9V1CggAkhMzhXSg8ECp7fHrWQX2G1chf");
}

lazy_static! {
    /// Map of feature identifiers to user-visible description
    pub static ref FEATURE_NAMES: HashMap<Pubkey, &'static str> = [
        (instructions_sysvar_enabled::id(), "instructions sysvar"),
        (consistent_recent_blockhashes_sysvar::id(), "consistent recentblockhashes sysvar"),
        (deprecate_rewards_sysvar::id(), "deprecate unused rewards sysvar"),
        (pico_inflation::id(), "pico inflation"),
        (full_inflation::devnet_and_testnet::id(), "full inflation on devnet and testnet"),
        (safe_token_v2_multisig_fix::id(), "safe-token multisig fix"),
        (no_overflow_rent_distribution::id(), "no overflow rent distribution"),
        (filter_stake_delegation_accounts::id(), "filter stake_delegation_accounts #14062"),
        (bpf_loader_upgradeable_program::id(), "upgradeable bpf loader"),
        (stake_program_v3::id(), "solana_stake_program v3"),
        (require_custodian_for_locked_stake_authorize::id(), "require custodian to authorize withdrawer change for locked stake"),
        (safe_token_v2_self_transfer_fix::id(), "safe-token self-transfer fix"),
        (full_inflation::mainnet::certusone::enable::id(), "full inflation enabled by Certus One"),
        (full_inflation::mainnet::certusone::vote::id(), "community vote allowing Certus One to enable full inflation"),
        (warp_timestamp_again::id(), "warp timestamp again, adjust bounding to 25% fast 80% slow #15204"),
        (check_init_vote_data::id(), "check initialized Vote data"),
        (check_program_owner::id(), "limit programs to operating on accounts owned by itself"),
        (require_stake_for_gossip::id(), "require stakes for propagating crds values through gossip #15561"),
        (cpi_data_cost::id(), "charge the compute budget for data passed via CPI"),
        (upgradeable_close_instruction::id(), "close upgradeable buffer accounts"),
        (sysvar_via_syscall::id(), "provide sysvars via syscalls"),
        (enforce_aligned_host_addrs::id(), "enforce aligned host addresses"),
        (set_upgrade_authority_via_cpi_enabled::id(), "set upgrade authority instruction via cpi calls for upgradable programs"),
        (update_data_on_realloc::id(), "Retain updated data values modified after realloc via CPI"),
        (keccak256_syscall_enabled::id(), "keccak256 syscall"),
        (stake_program_v4::id(), "solana_stake_program v4"),
        (memory_ops_syscalls::id(), "add syscalls for memory operations"),
        (secp256k1_recover_syscall_enabled::id(), "secp256k1_recover syscall"),
        (add_missing_program_error_mappings::id(), "add missing program error mappings"),
        (system_transfer_zero_check::id(), "perform all checks for transfers of 0 lamports"),
        (dedupe_config_program_signers::id(), "dedupe config program signers"),
        (verify_tx_signatures_len::id(), "prohibit extra transaction signatures"),
        (deterministic_shred_seed_enabled::id(), "deterministic shred seed"),
        (vote_stake_checked_instructions::id(), "vote/state program checked instructions #18345"),
        (updated_verify_policy::id(), "Update verify policy"),
        (neon_evm_compute_budget::id(), "bump neon_evm's compute budget"),
        (rent_for_sysvars::id(), "collect rent from accounts owned by sysvars"),
        (libsecp256k1_0_5_upgrade_enabled::id(), "upgrade libsecp256k1 to v0.5.0"),
        (stop_verify_mul64_imm_nonzero::id(), "Sets rbpf vm config verify_mul64_imm_nonzero to false"),
        (start_verify_shift32_imm::id(), "sets rbpf vm config verify_shift32_imm to true"),
        (merge_nonce_error_into_system_error::id(), "merge NonceError into SystemError"),
        (safe_token_v2_set_authority_fix::id(), "safe-token set_authority fix"),
        (stake_merge_with_unmatched_credits_observed::id(), "allow merging active stakes with unmatched credits_observed #18985"),
        (mem_overlap_fix::id(), "Memory overlap fix"),
        (close_upgradeable_program_accounts::id(), "enable closing upgradeable program accounts"),
        (stake_program_advance_activating_credits_observed::id(), "Enable advancing credits observed for activation epoch #19309"),
        (demote_program_write_locks::id(), "demote program write locks to readonly, except when upgradeable loader present #19593 #20265"),
        (allow_native_ids::id(), "allow native program ids in program derived addresses"),
        (check_seed_length::id(), "Check program address seed lengths"),
        (fix_write_privs::id(), "fix native invoke write privileges"),
        (reduce_required_deploy_balance::id(), "reduce required payer balance for program deploys"),
        (stakes_remove_delegation_if_inactive::id(), "remove delegations from stakes cache when inactive"),
        (send_to_tpu_vote_port::id(), "Send votes to the tpu vote port"),
        (optimize_epoch_boundary_updates::id(), "Optimize epoch boundary updates"),
        (tx_wide_compute_cap::id(), "Transaction wide compute cap"),
        (gate_large_block::id(), "validator checks block cost against max limit in realtime, reject if exceeds."),
        (remove_native_loader::id(), "Remove support for the native loader"),
        (return_data_syscall_enabled::id(), "enable sol_{set,get}_return_data syscall"),
        (sol_log_data_syscall_enabled::id(), "enable sol_log_data syscall"),
        (ed25519_program_enabled::id(), "enable builtin ed25519 signature verify program"),
        (requestable_heap_size::id(), "Requestable heap frame size"),
        (add_compute_budget_program::id(), "Add compute_budget_program"),
        (reject_deployment_of_unresolved_syscalls::id(), "Reject deployment of programs with unresolved syscall symbols"),
        (reject_section_virtual_address_file_offset_mismatch::id(), "enforce section virtual addresses and file offsets in ELF to be equal"),
        (reject_all_elf_rw::id(), "reject all read-write data in program elfs"),
        (safe_token_v3_3_0_release::id(), "safe-token v3.3.0 release"),
        (reject_non_rent_exempt_vote_withdraws::id(), "fail vote withdraw instructions which leave the account non-rent-exempt"),
        (evict_invalid_stakes_cache_entries::id(), "evict invalid stakes cache entries on epoch boundaries"),
        /*************** ADD NEW FEATURES HERE ***************/
    ]
    .iter()
    .cloned()
    .collect();

    /// Unique identifier of the current software's feature set
    pub static ref ID: Hash = {
        let mut hasher = Hasher::default();
        let mut feature_ids = FEATURE_NAMES.keys().collect::<Vec<_>>();
        feature_ids.sort();
        for feature in feature_ids {
            hasher.hash(feature.as_ref());
        }
        hasher.result()
    };
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct FullInflationFeaturePair {
    pub vote_id: Pubkey, // Feature that grants the candidate the ability to enable full inflation
    pub enable_id: Pubkey, // Feature to enable full inflation by the candidate
}

lazy_static! {
    /// Set of feature pairs that once enabled will trigger full inflation
    pub static ref FULL_INFLATION_FEATURE_PAIRS: HashSet<FullInflationFeaturePair> = [
        FullInflationFeaturePair {
            vote_id: full_inflation::mainnet::certusone::vote::id(),
            enable_id: full_inflation::mainnet::certusone::enable::id(),
        },
    ]
    .iter()
    .cloned()
    .collect();
}

/// `FeatureSet` holds the set of currently active/inactive runtime features
#[derive(AbiExample, Debug, Clone)]
pub struct FeatureSet {
    pub active: HashMap<Pubkey, Slot>,
    pub inactive: HashSet<Pubkey>,
}
impl Default for FeatureSet {
    fn default() -> Self {
        // All features disabled
        Self {
            active: HashMap::new(),
            inactive: FEATURE_NAMES.keys().cloned().collect(),
        }
    }
}
impl FeatureSet {
    pub fn is_active(&self, feature_id: &Pubkey) -> bool {
        self.active.contains_key(feature_id)
    }

    pub fn activated_slot(&self, feature_id: &Pubkey) -> Option<Slot> {
        self.active.get(feature_id).copied()
    }

    /// List of enabled features that trigger full inflation
    pub fn full_inflation_features_enabled(&self) -> HashSet<Pubkey> {
        let mut hash_set = FULL_INFLATION_FEATURE_PAIRS
            .iter()
            .filter_map(|pair| {
                if self.is_active(&pair.vote_id) && self.is_active(&pair.enable_id) {
                    Some(pair.enable_id)
                } else {
                    None
                }
            })
            .collect::<HashSet<_>>();

        if self.is_active(&full_inflation::devnet_and_testnet::id()) {
            hash_set.insert(full_inflation::devnet_and_testnet::id());
        }
        hash_set
    }

    /// All features enabled, useful for testing
    pub fn all_enabled() -> Self {
        Self {
            active: FEATURE_NAMES.keys().cloned().map(|key| (key, 0)).collect(),
            inactive: HashSet::new(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_full_inflation_features_enabled_devnet_and_testnet() {
        let mut feature_set = FeatureSet::default();
        assert!(feature_set.full_inflation_features_enabled().is_empty());
        feature_set
            .active
            .insert(full_inflation::devnet_and_testnet::id(), 42);
        assert_eq!(
            feature_set.full_inflation_features_enabled(),
            [full_inflation::devnet_and_testnet::id()]
                .iter()
                .cloned()
                .collect()
        );
    }

    #[test]
    fn test_full_inflation_features_enabled() {
        // Normal sequence: vote_id then enable_id
        let mut feature_set = FeatureSet::default();
        assert!(feature_set.full_inflation_features_enabled().is_empty());
        feature_set
            .active
            .insert(full_inflation::mainnet::certusone::vote::id(), 42);
        assert!(feature_set.full_inflation_features_enabled().is_empty());
        feature_set
            .active
            .insert(full_inflation::mainnet::certusone::enable::id(), 42);
        assert_eq!(
            feature_set.full_inflation_features_enabled(),
            [full_inflation::mainnet::certusone::enable::id()]
                .iter()
                .cloned()
                .collect()
        );

        // Backwards sequence: enable_id and then vote_id
        let mut feature_set = FeatureSet::default();
        assert!(feature_set.full_inflation_features_enabled().is_empty());
        feature_set
            .active
            .insert(full_inflation::mainnet::certusone::enable::id(), 42);
        assert!(feature_set.full_inflation_features_enabled().is_empty());
        feature_set
            .active
            .insert(full_inflation::mainnet::certusone::vote::id(), 42);
        assert_eq!(
            feature_set.full_inflation_features_enabled(),
            [full_inflation::mainnet::certusone::enable::id()]
                .iter()
                .cloned()
                .collect()
        );
    }
}
