use anchor_lang::prelude::*;

use crate::{constants::SEED, state::Counter};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        space = 8 + Counter::INIT_SPACE,
        seeds = [SEED.as_bytes(), signer.key().as_ref()],
        bump
    )]
    pub counter: Account<'info, Counter>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Initialize>, count: u64) -> Result<()> {
    ctx.accounts.counter.count = count;
    ctx.accounts.counter.bump = ctx.bumps.counter;
    Ok(())
}
