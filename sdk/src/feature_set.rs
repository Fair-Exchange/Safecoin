use lazy_static::lazy_static;
use solana_sdk::{
    clock::Slot,
    hash::{Hash, Hasher},
    pubkey::Pubkey,
};
use std::collections::{HashMap, HashSet};

pub mod instructions_sysvar_enabled {
    solana_sdk::declare_id!("7TfFp6Tf2XqXQQfx16qvXbjekXtn68kiQj9pPfXZ5Bua");
}

pub mod consistent_recent_blockhashes_sysvar {
    solana_sdk::declare_id!("ApdySrEmykK3PBLExdN2ehGCRPyVT6athFgu4H7H8e9J");
}

pub mod deprecate_rewards_sysvar {
    solana_sdk::declare_id!("GVriKcZATdgSzmhmu9PtimfheaKYksK5mhgoN5fbRhXg");
}

pub mod pico_inflation {
    solana_sdk::declare_id!("HC5H1tGb2RBRbrECSKmBVjwTsEEJziTTRsm3XSdTefrJ");
}

pub mod full_inflation {
    pub mod devnet_and_testnet {
        solana_sdk::declare_id!("JC1GQujpgHz92Am65FxcxByqqDGTdYJs6jLjgebnDpJF");
    }

    pub mod mainnet {
        pub mod certusone {
            pub mod vote {
                solana_sdk::declare_id!("7Ra6gKzEBFyYJ7fdsrCuzihZcxKu7AbmDy2FBEQFzap2");
            }
            pub mod enable {
                solana_sdk::declare_id!("54ZqXvMyCKShnBrGj2ni6m4kXTjRUrkc21dFoT9npJZk");
            }
        }
    }
}

pub mod spl_token_v2_multisig_fix {
    solana_sdk::declare_id!("EwSWSBRpzYZSEqdZ215WMmnce6WiEsk57rSEB3e7ghh6");
}

pub mod no_overflow_rent_distribution {
    solana_sdk::declare_id!("9TyDRDhs933rTCWGwzSUTSx1XeJT14sc17o1cNQzUaBq");
}

pub mod filter_stake_delegation_accounts {
    solana_sdk::declare_id!("HpGqShCRhP7QwMBXTs1KbATiHWa383EUWjg3kbQjN2Kf");
}

pub mod bpf_loader_upgradeable_program {
    solana_sdk::declare_id!("Cv6gGxiakDF6nd9Sxx53MbC4kij69qXpap8guiC6aK9U");
}

pub mod stake_program_v3 {
    solana_sdk::declare_id!("6tYrCsaWbGqgeW9tN3NRbViw6BBLYjnNsJBqLJZZoo5B");
}

pub mod require_custodian_for_locked_stake_authorize {
    solana_sdk::declare_id!("FKWSvfcXATHSBBNvr5VE6ns4tNsTG3EGzcDw2xVtowZQ");
}

pub mod spl_token_v2_self_transfer_fix {
    solana_sdk::declare_id!("2XDc17ZmSTbpqV3B5fmEGac4CKCYnbJj7vnfASvdzqyN");
}

pub mod warp_timestamp_again {
    solana_sdk::declare_id!("EgfbGcUzJotpXW8H4TnhVwDbrjm75xecKnrSAfqv4YvW");
}

pub mod check_init_vote_data {
    solana_sdk::declare_id!("CfH5RJxvjkQ1LaBSrnCDnYvHZtL6FuTQ751myi6DJEb5");
}

pub mod check_program_owner {
    solana_sdk::declare_id!("93ksjAw2xaVgWcBaqQSasj6hNiX3yUkPLpVZ8hoE2jE6");
}

pub mod require_stake_for_gossip {
    solana_sdk::declare_id!("7WM5jgvcKh9yyEj4NhG6qt1MvahRB6mhkodaY5KQwT4k");
}

pub mod cpi_data_cost {
    solana_sdk::declare_id!("6U62sD7ffNehUa7QLia5DUPNmFvGeZgrGLnRPo5Jdw9v");
}

pub mod upgradeable_close_instruction {
    solana_sdk::declare_id!("EzdQuAfpfg1pDTLyj52PLMPJDDCo4aqzncsSprFvMvU5");
}

pub mod sysvar_via_syscall {
    solana_sdk::declare_id!("92ep2YChAdo4XiDPmNkRcAfJZuRv46n5ko9z6S512awn");
}

pub mod enforce_aligned_host_addrs {
    solana_sdk::declare_id!("6yrRvLK2AT8U9dq4aSArH3MSRdHbSYQsCd99zHxeAXJ8");
}

pub mod set_upgrade_authority_via_cpi_enabled {
    solana_sdk::declare_id!("8voJR1noBKPVBgE7BV6g3DCu4FJshGmCs1dRPuKnyJH5");
}

pub mod update_data_on_realloc {
    solana_sdk::declare_id!("DcTaQRy9wRBGywHkFStEnBW8V7yiJkGADXYbByjM3UEQ");
}

pub mod keccak256_syscall_enabled {
    solana_sdk::declare_id!("8ubpzsyCpCcDVccAnPNuQV439SPw5xdCpFryBMU8wPv1");
}

pub mod stake_program_v4 {
    solana_sdk::declare_id!("8Sgh17Pmrt6kKCJuNFGzJTnwRSMt1ELCSBuYThLznCYR");
}

pub mod memory_ops_syscalls {
    solana_sdk::declare_id!("45aF3i2bmR2e9bPnZNpyRVZexjbaYKyq7f4PG59jhXxg");
}

pub mod secp256k1_recover_syscall_enabled {
    solana_sdk::declare_id!("6RvdSWHh8oh72Dp7wMTS2DBkf3fRPtChfNrAo3cZZoXJ");
}

pub mod add_missing_program_error_mappings {
    solana_sdk::declare_id!("3QEUpjhgPEt92nz3Mqf6pABkHPGCQwSvKtyGMq4SuQyL");
}

pub mod system_transfer_zero_check {
    solana_sdk::declare_id!("ACrERgP7eXsQquYUNQwASR884izTTXnR2jDnxjWRPEde");
}

pub mod dedupe_config_program_signers {
    solana_sdk::declare_id!("Ffpg1Q73ocztQKMTbkTMeTAVpcGUb95u2aUaCeFQ47NY");
}

pub mod deterministic_shred_seed_enabled {
    solana_sdk::declare_id!("FjSRMpFe7mofQ3WrEMT7Smjk2sME1XdAoRxcv55V6M44");
}

pub mod verify_tx_signatures_len {
    solana_sdk::declare_id!("EVW9B5xD9FFK7vw1SBARwMA4s5eRo5eKJdKpsBikzKBz");
}

pub mod vote_stake_checked_instructions {
    solana_sdk::declare_id!("BjfQ729TvPN84xsvsGMUsRLAqGc1ttSCQ6C5Zd3rUg2N");
}

pub mod updated_verify_policy {
    solana_sdk::declare_id!("8JWHY3gNMHMMia4smn5fa6KH91znav6qUzLMAJVNaNoY");
}

pub mod neon_evm_compute_budget {
    solana_sdk::declare_id!("GLrVvDPkQi5PMYUrsYWT9doZhSHr1BVZXqj5DbFps3rS");
}

pub mod rent_for_sysvars {
    solana_sdk::declare_id!("BKCPBQQBZqggVnFso5nQ8rQ4RwwogYwjuUt9biBjxwNF");
}

pub mod libsecp256k1_0_5_upgrade_enabled {
    solana_sdk::declare_id!("EVHL8iX15Gf6PpjdMd7pqPivjPQ2LLbK9fkGsrsGWy9r");
}

pub mod merge_nonce_error_into_system_error {
    solana_sdk::declare_id!("4n5Ko6ax8yLi21CXoBMFbCy52QydH7jpy42W5df7GZqT");
}

pub mod spl_token_v2_set_authority_fix {
    solana_sdk::declare_id!("Cb3jN13cfCNDV9dp36djNpcZXF7r82UAE4U1tZjXnFx5");
}

pub mod stake_merge_with_unmatched_credits_observed {
    solana_sdk::declare_id!("meRgp4ArRPhD3KtCY9c5yAf2med7mBLsjKTPeVUHqBL");
}

pub mod gate_large_block {
    solana_sdk::declare_id!("2ry7ygxiYURULZCrypHhveanvP5tzZ4toRwVp89oCNSj");
}

pub mod mem_overlap_fix {
    solana_sdk::declare_id!("vXDCFK7gphrEmyf5VnKgLmqbdJ4UxD2eZH1qbdouYKF");
}

pub mod close_upgradeable_program_accounts {
    solana_sdk::declare_id!("EQMtCuSAkMVF9ZdhGuABtgvyXJLtSRF5AQKv1RNsrhj7");
}

pub mod stake_program_advance_activating_credits_observed {
    solana_sdk::declare_id!("SAdVFw3RZvzbo6DvySbSdBnHN4gkzSTH9dSxesyKKPj");
}

pub mod demote_program_write_locks {
    solana_sdk::declare_id!("3E3jV7v9VcdJL8iYZUMax9DiDno8j7EWUVbhm9RtShj2");
}

pub mod allow_native_ids {
    solana_sdk::declare_id!("GVnDbNkECwrzLM7aVBGWpBYo3yH1ACaXB4ottNX8pedZ");
}

pub mod check_seed_length {
    solana_sdk::declare_id!("8HYXgkoKGreAMA3MfJkdjbKNVbfZRQP3jqFpa7iqN4v7");
}

pub mod fix_write_privs {
    solana_sdk::declare_id!("7Tr5C1tdcCeBVD8jxtHYnvjL1DGdFboYBHCJkEFdenBb");
}

pub mod reduce_required_deploy_balance {
    solana_sdk::declare_id!("EBeznQDjcPG8491sFsKZYBi5S5jTVXMpAKNDJMQPS2kq");
}

pub mod stakes_remove_delegation_if_inactive {
    solana_sdk::declare_id!("HFpdDDNQjvcXnXKec697HDDsyk6tFoWS2o8fkxuhQZpL");
}

pub mod send_to_tpu_vote_port {
    solana_sdk::declare_id!("C5fh68nJ7uyKAuYZg2x9sEQ5YrVf3dkW6oojNBSc3Jvo");
}

pub mod optimize_epoch_boundary_updates {
    solana_sdk::declare_id!("265hPS8k8xJ37ot82KEgjRunsUp5w4n4Q4VwwiN9i9ps");
}

lazy_static! {
    /// Map of feature identifiers to user-visible description
    pub static ref FEATURE_NAMES: HashMap<Pubkey, &'static str> = [
        (instructions_sysvar_enabled::id(), "instructions sysvar"),
        (consistent_recent_blockhashes_sysvar::id(), "consistent recentblockhashes sysvar"),
        (deprecate_rewards_sysvar::id(), "deprecate unused rewards sysvar"),
        (pico_inflation::id(), "pico inflation"),
        (full_inflation::devnet_and_testnet::id(), "full inflation on devnet and testnet"),
        (spl_token_v2_multisig_fix::id(), "safe-token multisig fix"),
        (no_overflow_rent_distribution::id(), "no overflow rent distribution"),
        (filter_stake_delegation_accounts::id(), "filter stake_delegation_accounts #14062"),
        (bpf_loader_upgradeable_program::id(), "upgradeable bpf loader"),
        (stake_program_v3::id(), "solana_stake_program v3"),
        (require_custodian_for_locked_stake_authorize::id(), "require custodian to authorize withdrawer change for locked stake"),
        (spl_token_v2_self_transfer_fix::id(), "safe-token self-transfer fix"),
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
        (merge_nonce_error_into_system_error::id(), "merge NonceError into SystemError"),
        (spl_token_v2_set_authority_fix::id(), "safe-token set_authority fix"),
        (stake_merge_with_unmatched_credits_observed::id(), "allow merging active stakes with unmatched credits_observed #18985"),
        (gate_large_block::id(), "validator checks block cost against max limit in realtime, reject if exceeds."),
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
