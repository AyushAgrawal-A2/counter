use pinocchio::error::ProgramError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CounterError {
    #[error("Invalid counter account")]
    InvalidCounterAccount,
    #[error("Increment overflow")]
    IncrementOverflow,
    #[error("Decrement underflow")]
    DecrementUnderflow,
    #[error("Lamports overflow")]
    LamportsOverflow,
}
impl From<CounterError> for ProgramError {
    fn from(e: CounterError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
