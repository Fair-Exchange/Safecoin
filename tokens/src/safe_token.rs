use {
    crate::{
        args::{DistributeTokensArgs, SafeTokenArgs},
        commands::{get_fees_for_messages, Allocation, Error, FundingSource},
    },
    console::style,
    safecoin_account_decoder::parse_token::{
        pubkey_from_safe_token, real_number_string, real_number_string_trimmed, safe_token_pubkey,
    },
    safecoin_client::rpc_client::RpcClient,
    safecoin_sdk::{instruction::Instruction, message::Message, native_token::lamports_to_sol},
    safecoin_transaction_status::parse_token::safe_token_instruction,
    safe_associated_token_account::{
        get_associated_token_address, instruction::create_associated_token_account,
    },
    safe_token::{
        safecoin_program::program_pack::Pack,
        state::{Account as SafeTokenAccount, Mint},
    },
};

pub fn update_token_args(client: &RpcClient, args: &mut Option<SafeTokenArgs>) -> Result<(), Error> {
    if let Some(safe_token_args) = args {
        let sender_account = client
            .get_account(&safe_token_args.token_account_address)
            .unwrap_or_default();
        let mint_address =
            pubkey_from_safe_token(&SafeTokenAccount::unpack(&sender_account.data)?.mint);
        safe_token_args.mint = mint_address;
        update_decimals(client, args)?;
    }
    Ok(())
}

pub fn update_decimals(client: &RpcClient, args: &mut Option<SafeTokenArgs>) -> Result<(), Error> {
    if let Some(safe_token_args) = args {
        let mint_account = client.get_account(&safe_token_args.mint).unwrap_or_default();
        let mint = Mint::unpack(&mint_account.data)?;
        safe_token_args.decimals = mint.decimals;
    }
    Ok(())
}

pub fn safe_token_amount(amount: f64, decimals: u8) -> u64 {
    (amount * 10_usize.pow(decimals as u32) as f64) as u64
}

pub fn build_safe_token_instructions(
    allocation: &Allocation,
    args: &DistributeTokensArgs,
    do_create_associated_token_account: bool,
) -> Vec<Instruction> {
    let safe_token_args = args
        .safe_token_args
        .as_ref()
        .expect("safe_token_args must be some");
    let wallet_address = allocation.recipient.parse().unwrap();
    let associated_token_address =
        get_associated_token_address(&wallet_address, &safe_token_pubkey(&safe_token_args.mint));
    let mut instructions = vec![];
    if do_create_associated_token_account {
        let create_associated_token_account_instruction = create_associated_token_account(
            &safe_token_pubkey(&args.fee_payer.pubkey()),
            &wallet_address,
            &safe_token_pubkey(&safe_token_args.mint),
            &safe_token::id(),
        );
        instructions.push(safe_token_instruction(
            create_associated_token_account_instruction,
        ));
    }
    let spl_instruction = safe_token::instruction::transfer_checked(
        &safe_token::id(),
        &safe_token_pubkey(&safe_token_args.token_account_address),
        &safe_token_pubkey(&safe_token_args.mint),
        &associated_token_address,
        &safe_token_pubkey(&args.sender_keypair.pubkey()),
        &[],
        allocation.amount,
        safe_token_args.decimals,
    )
    .unwrap();
    instructions.push(safe_token_instruction(spl_instruction));
    instructions
}

pub fn check_safe_token_balances(
    messages: &[Message],
    allocations: &[Allocation],
    client: &RpcClient,
    args: &DistributeTokensArgs,
    created_accounts: u64,
) -> Result<(), Error> {
    let safe_token_args = args
        .safe_token_args
        .as_ref()
        .expect("safe_token_args must be some");
    let allocation_amount: u64 = allocations.iter().map(|x| x.amount).sum();
    let fees = get_fees_for_messages(messages, client)?;

    let token_account_rent_exempt_balance =
        client.get_minimum_balance_for_rent_exemption(SafeTokenAccount::LEN)?;
    let account_creation_amount = created_accounts * token_account_rent_exempt_balance;
    let fee_payer_balance = client.get_balance(&args.fee_payer.pubkey())?;
    if fee_payer_balance < fees + account_creation_amount {
        return Err(Error::InsufficientFunds(
            vec![FundingSource::FeePayer].into(),
            lamports_to_sol(fees + account_creation_amount).to_string(),
        ));
    }
    let source_token_account = client
        .get_account(&safe_token_args.token_account_address)
        .unwrap_or_default();
    let source_token = SafeTokenAccount::unpack(&source_token_account.data)?;
    if source_token.amount < allocation_amount {
        return Err(Error::InsufficientFunds(
            vec![FundingSource::SafeTokenAccount].into(),
            real_number_string_trimmed(allocation_amount, safe_token_args.decimals),
        ));
    }
    Ok(())
}

pub fn print_token_balances(
    client: &RpcClient,
    allocation: &Allocation,
    safe_token_args: &SafeTokenArgs,
) -> Result<(), Error> {
    let address = allocation.recipient.parse().unwrap();
    let expected = allocation.amount;
    let associated_token_address = get_associated_token_address(
        &safe_token_pubkey(&address),
        &safe_token_pubkey(&safe_token_args.mint),
    );
    let recipient_account = client
        .get_account(&pubkey_from_safe_token(&associated_token_address))
        .unwrap_or_default();
    let (actual, difference) = if let Ok(recipient_token) =
        SafeTokenAccount::unpack(&recipient_account.data)
    {
        let actual_ui_amount = real_number_string(recipient_token.amount, safe_token_args.decimals);
        let delta_string =
            real_number_string(recipient_token.amount - expected, safe_token_args.decimals);
        (
            style(format!("{:>24}", actual_ui_amount)),
            format!("{:>24}", delta_string),
        )
    } else {
        (
            style("Associated token account not yet created".to_string()).yellow(),
            "".to_string(),
        )
    };
    println!(
        "{:<44}  {:>24}  {:>24}  {:>24}",
        allocation.recipient,
        real_number_string(expected, safe_token_args.decimals),
        actual,
        difference,
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    // The following unit tests were written for v1.4 using the ProgramTest framework, passing its
    // BanksClient into the `safecoin-tokens` methods. With the revert to RpcClient in this module
    // (https://github.com/fair-exchange/safecoin/pull/13623), that approach was no longer viable.
    // These tests were removed rather than rewritten to avoid accruing technical debt. Once a new
    // rpc/client framework is implemented, they should be restored.
    //
    // async fn test_process_safe_token_allocations()
    // async fn test_process_safe_token_transfer_amount_allocations()
    // async fn test_check_safe_token_balances()
    //
    // https://github.com/fair-exchange/safecoin/blob/5511d52c6284013a24ced10966d11d8f4585799e/tokens/src/safe_token.rs#L490-L685
}
