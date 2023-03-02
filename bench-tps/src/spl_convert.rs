use {
    solana_sdk::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
    },
    spl_instruction_padding::solana_program::{
        instruction::{AccountMeta as SplAccountMeta, Instruction as SplInstruction},
        pubkey::Pubkey as SplPubkey,
    },
};

pub trait FromOtherSafecoin<T> {
    fn from(_: T) -> Self;
}

macro_rules! impl_pubkey_conversion {
    ($S:ty, $L:ty) => {
        impl FromOtherSafecoin<$S> for $L {
            fn from(pubkey: $S) -> Self {
                Self::new_from_array(pubkey.to_bytes())
            }
        }
    };
}
impl_pubkey_conversion!(SplPubkey, Pubkey);
impl_pubkey_conversion!(Pubkey, SplPubkey);

macro_rules! impl_account_meta_conversion {
    ($S:ty, $L:ty) => {
        impl FromOtherSafecoin<$S> for $L {
            fn from(meta: $S) -> Self {
                Self {
                    pubkey: FromOtherSafecoin::from(meta.pubkey),
                    is_signer: meta.is_signer,
                    is_writable: meta.is_writable,
                }
            }
        }
    };
}
impl_account_meta_conversion!(SplAccountMeta, AccountMeta);
impl_account_meta_conversion!(AccountMeta, SplAccountMeta);

macro_rules! impl_instruction_conversion {
    ($S: ty, $L:ty) => {
        impl FromOtherSafecoin<$S> for $L {
            fn from(instruction: $S) -> Self {
                Self {
                    program_id: FromOtherSafecoin::from(instruction.program_id),
                    accounts: instruction
                        .accounts
                        .into_iter()
                        .map(|meta| FromOtherSafecoin::from(meta))
                        .collect(),
                    data: instruction.data,
                }
            }
        }
    };
}
impl_instruction_conversion!(SplInstruction, Instruction);
impl_instruction_conversion!(Instruction, SplInstruction);
