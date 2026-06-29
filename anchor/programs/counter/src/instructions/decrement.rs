use anchor_lang::prelude::*;

use crate::{constants::SEED, error::CounterError, state::Counter};

#[derive(Accounts)]
pub struct Decrement<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds = [SEED.as_bytes(), signer.key().as_ref()],
        bump = counter.bump
    )]
    pub counter: Account<'info, Counter>,
}

pub fn handler(ctx: Context<Decrement>) -> Result<()> {
    ctx.accounts.counter.count = ctx
        .accounts
        .counter
        .count
        .checked_sub(1)
        .ok_or(CounterError::DecrementUnderflow)?;
    Ok(())
}
