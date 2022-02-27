use anchor_lang::prelude::*;

use solana_program::entrypoint::ProgramResult;

declare_id!("99DwSCgBXD3DsAq4CUqtXihiUT74VxMK1sUzNStKzN6e");

#[program]
pub mod defi_app {

    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        base_account.total_balance = 0;
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: String) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;

        let parsed_amount = amount.parse::<u64>().unwrap();

        let deposit_interest = 0.01 * parsed_amount as f64;

        let token = Token {
            balance: parsed_amount,
            user_address: *user.to_account_info().key,
        };

        base_account.is_deposit = true;
        base_account.total_balance += parsed_amount;
        base_account.deposit_interest = deposit_interest;
        base_account.message = "Deposit success".to_string();
        base_account.tokens.push(token);

        Ok(())
    }

    pub fn borrow(ctx: Context<Borrow>, amount: String) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;

        let deposited_amount = base_account.total_balance;
        let borrowable_amount = deposited_amount / 2;

        let parsed_amount = amount.parse::<u64>().unwrap();

        let borrow_interest = 0.02 * parsed_amount as f64;

        let token = Token {
            balance: parsed_amount,
            user_address: *user.to_account_info().key,
        };

        if base_account.is_deposit == true {
            if parsed_amount <= borrowable_amount {
                base_account.total_balance -= parsed_amount;
                base_account.borrow_interest = borrow_interest;
                base_account.message = "Borrow success".to_string();
                base_account.tokens.push(token);
            } else {
                base_account.message =
                    "Can't perform transaction as borrow amount is greated that deposited amount"
                        .to_string()
            }
        } else {
            base_account.message = String::from("Amount should be deposited first")
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer=user, space=9000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct Borrow<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct Token {
    pub balance: u64,
    pub user_address: Pubkey,
}

#[account]
pub struct BaseAccount {
    pub total_balance: u64,
    pub is_deposit: bool,
    pub tokens: Vec<Token>,
    pub message: String,
    pub borrow_interest: f64,
    pub deposit_interest: f64,
}
