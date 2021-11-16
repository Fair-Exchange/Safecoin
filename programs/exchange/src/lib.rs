#![allow(clippy::integer_arithmetic)]
pub mod exchange_instruction;
pub mod exchange_processor;
pub mod exchange_state;

#[macro_use]
extern crate solana_metrics;

use crate::exchange_processor::process_instruction;

safecoin_sdk::declare_program!(
    "Exchange11111111111111111111111111111111111",
    solana_exchange_program,
    process_instruction
);

pub mod faucet {
    safecoin_sdk::declare_id!("ExchangeFaucet11111111111111111111111111111");
}
