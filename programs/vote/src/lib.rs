#![cfg_attr(RUSTC_WITH_SPECIALIZATION, feature(min_specialization))]

pub mod vote_processor;
pub mod vote_state;
pub mod vote_transaction;

#[macro_use]
extern crate solana_metrics;

#[macro_use]
extern crate safecoin_frozen_abi_macro;

pub use safecoin_sdk::vote::{
    authorized_voters, error as vote_error, instruction as vote_instruction,
    program::{check_id, id},
};
