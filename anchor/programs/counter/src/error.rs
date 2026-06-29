use anchor_lang::prelude::*;

#[error_code]
pub enum CounterError {
    #[msg("Increment overflow")]
    IncrementOverflow,
    #[msg("Decrement underflow")]
    DecrementUnderflow,
}
