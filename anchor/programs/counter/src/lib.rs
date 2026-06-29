pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

use crate::instructions::*;

declare_id!("HsKpoYcxvixTmpUjyd4em6uWxaQcPQHMADRPNxM4S51E");

#[program]
pub mod counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, count: u64) -> Result<()> {
        initialize::handler(ctx, count)
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        increment::handler(ctx)
    }

    pub fn decrement(ctx: Context<Decrement>) -> Result<()> {
        decrement::handler(ctx)
    }

    pub fn close(ctx: Context<Close>) -> Result<()> {
        close::handler(ctx)
    }
}
