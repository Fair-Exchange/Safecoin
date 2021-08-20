use solana_sdk::{
    account::{Account, AccountSharedData},
    pubkey::Pubkey,
    rent::Rent,
};

mod spl_token {
    solana_sdk::declare_id!("7v5TwK92hUSqduoL3R8NtzTNfNzMA48nJL4mzPYMdDrD");
}
mod safe_memo_1_0 {
    solana_sdk::declare_id!("4DDUJ1rA8Vd7e6SFWanf4V8JnsfapjCGNutQYw8Vtt45");
}
mod safe_memo_3_0 {
    solana_sdk::declare_id!("9h7wfE8nxQ6YsRedqNHwroEZbA5bMAmNsh8GdxwBTtaV");
}
mod safe_associated_token_account {
    solana_sdk::declare_id!("CWyEp7dp1Cv3334j6gCci2UrrjA8Q98bYa7AwGBpZ6iJ");
}

static SPL_PROGRAMS: &[(Pubkey, &[u8])] = &[
    (spl_token::ID, include_bytes!("programs/spl_token-3.1.0.so")),
    (
        safe_memo_1_0::ID,
        include_bytes!("programs/safe_memo-1.0.0.so"),
    ),
    (
        safe_memo_3_0::ID,
        include_bytes!("programs/safe_memo-3.0.0.so"),
    ),
    (
        safe_associated_token_account::ID,
        include_bytes!("programs/spl_associated-token-account-1.0.1.so"),
    ),
];

pub fn spl_programs(rent: &Rent) -> Vec<(Pubkey, AccountSharedData)> {
    SPL_PROGRAMS
        .iter()
        .map(|(program_id, elf)| {
            (
                *program_id,
                AccountSharedData::from(Account {
                    lamports: rent.minimum_balance(elf.len()).min(1),
                    data: elf.to_vec(),
                    owner: solana_sdk::bpf_loader::id(),
                    executable: true,
                    rent_epoch: 0,
                }),
            )
        })
        .collect()
}
