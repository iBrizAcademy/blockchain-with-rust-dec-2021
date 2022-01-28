use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

// set latest quote and set counter on how many
// quotes have been created before it
#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct QuoteAccount {
    pub quote: String,
    pub counter: u32,
}

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    msg!("Quote of the day Solana Program");

    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;

    if account.owner != program_id {
        msg!("Quotes account does not have the correct program_id");
        return Err(ProgramError::IncorrectProgramId);
    }

    let mut quote_account = QuoteAccount::try_from_slice(&account.data.borrow())?;
    quote_account.counter += 1;
    quote_account.quote = String::from("Quote " + &quote_account.counter.to_string());

    quote_account.serialize(&mut &mut account.data.borrow_mut()[..])?;

    msg!("Quotes added : {} - {} times", quote_account.quote, quote_account.counter);
    Ok(())
}