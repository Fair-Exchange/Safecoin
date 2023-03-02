use solana_sdk::{
    account::{Account, AccountSharedData},
    bpf_loader_upgradeable::UpgradeableLoaderState,
    pubkey::Pubkey,
    rent::Rent,
};

mod safe_token {
    solana_sdk::declare_id!("ToKLx75MGim1d1jRusuVX8xvdvvbSDESVaNXpRA9PHN");
}
mod safe_token_2022 {
    solana_sdk::declare_id!("ZToGWcF1Qh9H7te1MmABiGsFUKvj5zXPQ2QnTqoHpHN");
}
mod safe_memo_1_0 {
    solana_sdk::declare_id!("MEMDqRW2fYAU19mcFnoDVoqG4Br4t7TdyWjjv38P6Nc");
}
mod safe_memo_3_0 {
    solana_sdk::declare_id!("MEMWKbqsjEB8o972BvDHExZFSauzGZKvB4xHDVPFowh");
}
mod safe_associated_token_account {
    solana_sdk::declare_id!("AToD9iqHSc2fhEP9Jp7UYA6mRjHQ4CTWyzCsw8X3tH7K");
}

static SPL_PROGRAMS: &[(Pubkey, Pubkey, &[u8])] = &[
    (
        safe_token::ID,
        solana_sdk::bpf_loader::ID,
        include_bytes!("programs/safe_token-3.5.0.so"),
    ),
    (
        safe_token_2022::ID,
        solana_sdk::bpf_loader_upgradeable::ID,
        include_bytes!("programs/safe_token_2022-0.5.0.so"),
    ),
    (
        safe_memo_1_0::ID,
        solana_sdk::bpf_loader::ID,
        include_bytes!("programs/safe_memo-1.0.0.so"),
    ),
    (
        safe_memo_3_0::ID,
        solana_sdk::bpf_loader::ID,
        include_bytes!("programs/safe_memo-3.0.0.so"),
    ),
    (
        safe_associated_token_account::ID,
        solana_sdk::bpf_loader::ID,
        include_bytes!("programs/safe_associated_token_account-1.1.1.so"),
    ),
];

pub fn spl_programs(rent: &Rent) -> Vec<(Pubkey, AccountSharedData)> {
    SPL_PROGRAMS
        .iter()
        .flat_map(|(program_id, loader_id, elf)| {
            let mut accounts = vec![];
            let data = if *loader_id == solana_sdk::bpf_loader_upgradeable::ID {
                let (programdata_address, _) =
                    Pubkey::find_program_address(&[program_id.as_ref()], loader_id);
                let mut program_data = bincode::serialize(&UpgradeableLoaderState::ProgramData {
                    slot: 0,
                    upgrade_authority_address: Some(Pubkey::default()),
                })
                .unwrap();
                program_data.extend_from_slice(elf);
                accounts.push((
                    programdata_address,
                    AccountSharedData::from(Account {
                        lamports: rent.minimum_balance(program_data.len()).max(1),
                        data: program_data,
                        owner: *loader_id,
                        executable: false,
                        rent_epoch: 0,
                    }),
                ));
                bincode::serialize(&UpgradeableLoaderState::Program {
                    programdata_address,
                })
                .unwrap()
            } else {
                elf.to_vec()
            };
            accounts.push((
                *program_id,
                AccountSharedData::from(Account {
                    lamports: rent.minimum_balance(data.len()).max(1),
                    data,
                    owner: *loader_id,
                    executable: true,
                    rent_epoch: 0,
                }),
            ));
            accounts.into_iter()
        })
        .collect()
}
