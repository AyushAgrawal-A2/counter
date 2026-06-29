use anchor_lang::prelude::*;

use crate::{constants::SEED, error::CounterError, state::Counter};

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds = [SEED.as_bytes(), signer.key().as_ref()],
        bump = counter.bump
    )]
    pub counter: Account<'info, Counter>,
}

pub fn handler(ctx: Context<Increment>) -> Result<()> {
    ctx.accounts.counter.count = ctx
        .accounts
        .counter
        .count
        .checked_add(1)
        .ok_or(CounterError::IncrementOverflow)?;
    Ok(())
}
