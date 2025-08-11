// This file is part of the Solana program for a simple "Hello World" example.
// It defines the program's entry point and declares its ID.


use anchor_lang::prelude::*;

declare_id!("4GzBh5jhsndYJeNaVjRpGqJqgzMVoZgiaZDHt4T6nCGB");

#[program]
pub mod hello_world {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, message: String) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;
        base_account.message = message;
        Ok(())
    }

    pub fn update_message(ctx: Context<UpdateMessage>, new_message: String) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;
        base_account.message = new_message;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 64)]
    pub base_account: Account<'info, MessageAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateMessage<'info> {
    #[account(mut)]
    pub base_account: Account<'info, MessageAccount>,
}

#[account]
pub struct MessageAccount {
    pub message: String,
}


