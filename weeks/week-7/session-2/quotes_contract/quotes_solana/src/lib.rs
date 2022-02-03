use std::str::from_utf8;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use std::io::ErrorKind::InvalidData;


// set latest quote and set counter on how many
// quotes have been created before it
#[derive(BorshDeserialize, BorshSerialize,PartialEq, Debug)]
pub struct QuoteAccount {
    pub counter: u32,
    pub quote: String,
}

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Quote of the day Solana Program");

    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;

    if account.owner != program_id {
        msg!("Quotes account does not have the correct program_id");
        return Err(ProgramError::IncorrectProgramId);
    }

    let mut quote_account = match QuoteAccount::try_from_slice(&account.data.borrow_mut()) {
        Ok(data) => data,
        Err(err) => {
            if err.kind() == InvalidData {
                msg!("InvalidData so initializing account data");
                QuoteAccount{counter: 0, quote:String::from("Quote0") }
            } else {
                panic!("Unknown error decoding account data {:?}", err)
            }
        }
    };

    let quote_string = from_utf8(instruction_data).map_err( |err| {
        msg!("Invalid UTF-8 from byte {}", err.valid_up_to());
        ProgramError::InvalidInstructionData
    })?;
    msg!("{} Bytes of new quote: {:?}", quote_string.len(), quote_string);
    quote_account.counter += 1;
    quote_account.quote = quote_string.to_string();

    quote_account.serialize(&mut &mut account.data.borrow_mut()[..])?;

    msg!("Quotes added : {} - {} times", quote_account.quote, quote_account.counter);
    Ok(())
}