//! Example Rust-based SBF program that tests rand behavior

#![allow(unreachable_code)]

extern crate safecoin_program;
use safecoin_program::{account_info::AccountInfo, entrypoint::ProgramResult, msg, pubkey::Pubkey};

safecoin_program::entrypoint!(process_instruction);
#[allow(clippy::unnecessary_wraps)]
fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    msg!("rand");
    Ok(())
}
