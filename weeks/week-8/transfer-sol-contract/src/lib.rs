use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Transfer some Sol to another account");

    let accounts_iter = &mut accounts.iter();

    let source_account = next_account_info(accounts_iter)?;
    let destination_account = next_account_info(accounts_iter)?;

    **source_account.try_borrow_mut_lamports()? -= 2;
    **destination_account.try_borrow_mut_lamports()? += 2;

    Ok(())
}