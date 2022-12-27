use std::borrow::Borrow;

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
};

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Hello, world!");

    let delta = u32::from_le_bytes(
        instruction_data[0..4]
            .try_into()
            .or(Err(ProgramError::InvalidInstructionData))?,
    );
    let (bump, _) = instruction_data[4..]
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;
    let accounts_iter = &mut accounts.iter();
    let funding_account = next_account_info(accounts_iter)?;
    let pda_account = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    if pda_account.try_borrow_data()?.len() < 4 {
        let new_size: usize = 4;
        // Initialize pda.
        let lamports_required = Rent::get()?.minimum_balance(new_size);
        let create_pda_account_ix = system_instruction::create_account(
            funding_account.key,
            pda_account.key,
            lamports_required,
            new_size as u64,
            program_id,
        );
        let signer_seeds: &[&[u8]; 2] = &[b"counter-seeds", &[*bump]];

        invoke_signed(
            &create_pda_account_ix,
            &[
                funding_account.clone(),
                pda_account.clone(),
                system_program.clone(),
            ],
            &[signer_seeds],
        )?;
    }
    let mut counter = u32::from_le_bytes(
        pda_account.try_borrow_data()?[0..4]
            .try_into()
            .or(Err(ProgramError::InvalidAccountData))?,
    );
    msg!("old counter value is {}", counter);
    counter += delta;
    msg!("now counter value is {}", counter);
    let byts = counter.to_le_bytes();
    let mut data = pda_account.try_borrow_mut_data()?;
    for i in 0..4 {
        data[i] = byts[i];
    }
    Ok(())
}
