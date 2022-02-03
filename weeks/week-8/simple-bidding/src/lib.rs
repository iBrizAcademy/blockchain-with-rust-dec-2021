
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    sysvar::Sysvar,
};
use std::io::ErrorKind::InvalidData;

entrypoint!(process_instruction);

#[derive(BorshDeserialize, BorshSerialize,PartialEq, Debug)]
pub struct Bid{
    highest_bidder: Pubkey, 
    highest_bid: u32,
    bid_counter: u32,
    description: String
}

fn create_bidding(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {

    let accounts_iter = &mut accounts.iter();
    let current_bidder = next_account_info(accounts_iter)?;

    let mut input_data = Bid::try_from_slice(&instruction_data)
        .expect("Instruction data serialization failed.");

    // check if the new bidding is greater than the old one

    let mut bidding_data = match Bid::try_from_slice(&current_bidder.data.borrow_mut()) {
            Ok(data) => data,
            Err(err) => {
                if err.kind() == InvalidData {
                    msg!("InvalidData so initializing account data");
                    Bid{highest_bidder: *current_bidder.key, highest_bid: 0, bid_counter: 0, description: String::from("000000000000000000000000000000")}
                } else {
                    panic!("Unknown error decoding account data {:?}", err)
                }
            }
        };


    // add validation that the bidder should have enough balance in its account greater than bid
    let rent_exemption = Rent::get()?.minimum_balance(current_bidder.data_len());
    if **current_bidder.lamports.borrow() < rent_exemption {
        msg!("The balance of current account should be more then rent_exemption");
        return Err(ProgramError::InsufficientFunds);
    }

    bidding_data.serialize(&mut &mut current_bidder.data.borrow_mut()[..])?;

    Ok(())
}

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if instruction_data.len() == 0 {
        return Err(ProgramError::InvalidInstructionData);
    }
    // bid an auction starting with 0.1 SOL = 100000000 Lamport
    return create_bidding(
        program_id,
        accounts,
        instruction_data
    );
}