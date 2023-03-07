use {
    crate::{inline_safe_associated_token_account, inline_safe_token, inline_safe_token_2022},
    solana_sdk::pubkey::Pubkey,
};

lazy_static! {
    /// Vector of static token & mint IDs
    pub static ref STATIC_IDS: Vec<Pubkey> = vec![
        inline_safe_associated_token_account::id(),
        inline_safe_associated_token_account::program_v1_1_0::id(),
        inline_safe_token::id(),
        inline_safe_token::native_mint::id(),
        inline_safe_token_2022::id(),
    ];
}
