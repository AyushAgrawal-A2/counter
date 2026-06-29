use anchor_lang::prelude::*;

use crate::{constants::SEED, state::Counter};

#[derive(Accounts)]
pub struct Close<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        close = signer,
        seeds = [SEED.as_bytes(), signer.key().as_ref()],
        bump = counter.bump,
    )]
    pub counter: Account<'info, Counter>,
}

pub fn handler(_ctx: Context<Close>) -> Result<()> {
    Ok(())
}
