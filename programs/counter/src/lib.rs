use anchor_lang::prelude::*;

declare_id!("Cco9Jh1g8w86iu1qzGYsLXuJKqoZMVGoaHE5yteLUbmF");

#[program]
mod counter {
    use super::*;

    pub fn create(ctx: Context<Create>, base_account_bump: u8) -> ProgramResult {
        ctx.accounts.base_account.bump = base_account_bump;
        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> ProgramResult {
        ctx.accounts.base_account.count += 1;
        Ok(())
    }
}

// Transaction instructions
#[derive(Accounts)]
#[instruction(base_account_bump: u8)]
pub struct Create<'info> {
    // #[account(init, payer = user, space = 16 + 16)] where is the space defined?
    #[account(init, seeds = [b"base_account".as_ref()], bump = base_account_bump, payer = user)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// Transaction instructions
#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut, seeds = [b"base_account".as_ref()], bump = base_account.bump)]
    pub base_account: Account<'info, BaseAccount>,
}

// An account that goes inside a transaction instruction
#[account]
#[derive(Default)]
pub struct BaseAccount {
    count: u64,
    bump: u8,
}
