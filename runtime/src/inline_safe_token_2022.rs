/// Partial SPL Token declarations inlined to avoid an external dependency on the safe-token-2022 crate
use crate::inline_safe_token::{self, GenericTokenAccount};

safecoin_sdk::declare_id!("ZToGWcF1Qh9H7te1MmABiGsFUKvj5zXPQ2QnTqoHpHN");

// `safe_token_program_2022::extension::AccountType::Account` ordinal value
const ACCOUNTTYPE_ACCOUNT: u8 = 2;

pub struct Account;
impl GenericTokenAccount for Account {
    fn valid_account_data(account_data: &[u8]) -> bool {
        inline_safe_token::Account::valid_account_data(account_data)
            || ACCOUNTTYPE_ACCOUNT
                == *account_data
                    .get(inline_safe_token::Account::get_packed_len())
                    .unwrap_or(&0)
    }
}
