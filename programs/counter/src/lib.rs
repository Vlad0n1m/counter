use anchor_lang::prelude::*;
use anchor_lang::system_program;

declare_id!("7cHBYYsn7X3Ep1hNRsw5ytgwYxmhJbG2nZKMXdzNsZwB");

#[program]
pub mod counter {
    use super::*;
    
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter_account = &mut ctx.accounts.counter; // ???
        counter_account.count = 0;
        return Ok(());
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        let counter_account = &mut ctx.accounts.counter;
        counter_account.count += 1;
        return Ok(());
    }
    
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + 8
    )]
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>
}

#[account]
pub struct Counter {
    pub count: u64,
}

