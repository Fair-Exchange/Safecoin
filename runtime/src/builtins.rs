use {
    crate::{
        bank::{Builtin, Builtins},
        system_instruction_processor,
    },
    safecoin_sdk::{
        feature_set,
        instruction::InstructionError,
        process_instruction::{stable_log, InvokeContext, ProcessInstructionWithContext},
        pubkey::Pubkey,
        stake, system_program,
    },
};

fn process_instruction_with_program_logging(
    process_instruction: ProcessInstructionWithContext,
    program_id: &Pubkey,
    instruction_data: &[u8],
    invoke_context: &mut dyn InvokeContext,
) -> Result<(), InstructionError> {
    let logger = invoke_context.get_logger();
    stable_log::program_invoke(&logger, program_id, invoke_context.invoke_depth());

    let result = process_instruction(program_id, instruction_data, invoke_context);

    match &result {
        Ok(()) => stable_log::program_success(&logger, program_id),
        Err(err) => stable_log::program_failure(&logger, program_id, err),
    }
    result
}

macro_rules! with_program_logging {
    ($process_instruction:expr) => {
        |program_id: &Pubkey, instruction_data: &[u8], invoke_context: &mut dyn InvokeContext| {
            process_instruction_with_program_logging(
                $process_instruction,
                program_id,
                instruction_data,
                invoke_context,
            )
        }
    };
}

/// Builtin programs that are always available
fn genesis_builtins() -> Vec<Builtin> {
    vec![
        Builtin::new(
            "system_program",
            system_program::id(),
            with_program_logging!(system_instruction_processor::process_instruction),
        ),
        Builtin::new(
            "vote_program",
            solana_vote_program::id(),
            with_program_logging!(solana_vote_program::vote_instruction::process_instruction),
        ),
        Builtin::new(
            "stake_program",
            stake::program::id(),
            with_program_logging!(solana_stake_program::stake_instruction::process_instruction),
        ),
        Builtin::new(
            "config_program",
            solana_config_program::id(),
            with_program_logging!(solana_config_program::config_processor::process_instruction),
        ),
        Builtin::new(
            "secp256k1_program",
            safecoin_sdk::secp256k1_program::id(),
            solana_secp256k1_program::process_instruction,
        ),
    ]
}

#[derive(AbiExample, Debug, Clone)]
pub enum ActivationType {
    NewProgram,
    NewVersion,
}

/// Builtin programs activated dynamically by feature
///
/// Note: If the feature_builtin is intended to replace another builtin program, it must have a new
/// name.
/// This is to enable the runtime to determine categorically whether the builtin update has
/// occurred, and preserve idempotency in Bank::add_native_program across genesis, snapshot, and
/// normal child Bank creation.
/// https://github.com/fair-exchange/safecoin/blob/84b139cc94b5be7c9e0c18c2ad91743231b85a0d/runtime/src/bank.rs#L1723
fn feature_builtins() -> Vec<(Builtin, Pubkey, ActivationType)> {
    vec![
        (
            Builtin::new(
                "compute_budget_program",
                safecoin_sdk::compute_budget::id(),
                solana_compute_budget_program::process_instruction,
            ),
            feature_set::add_compute_budget_program::id(),
            ActivationType::NewProgram,
        ),
        (
            Builtin::new(
                "ed25519_program",
                safecoin_sdk::ed25519_program::id(),
                solana_ed25519_program::process_instruction,
            ),
            feature_set::ed25519_program_enabled::id(),
            ActivationType::NewProgram,
        ),
    ]
}

pub(crate) fn get() -> Builtins {
    Builtins {
        genesis_builtins: genesis_builtins(),
        feature_builtins: feature_builtins(),
    }
}
