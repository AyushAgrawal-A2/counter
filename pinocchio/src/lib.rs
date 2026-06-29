#![no_std]

mod constants;
mod error;
mod instructions;
mod state;

use pinocchio::{
    entrypoint, error::ProgramError, nostd_panic_handler, AccountView, Address, ProgramResult,
};

use crate::instructions::{
    close::Close, decrement::Decrement, increment::Increment, initialize::Initialize,
};

pub const ID: Address = Address::from_str_const("22222222222222222222222222222222222222222222");

entrypoint!(process_instruction);
nostd_panic_handler!();

fn process_instruction(
    _program_id: &Address,
    accounts: &mut [AccountView],
    instruction_data: &[u8],
) -> ProgramResult {
    match instruction_data.split_first() {
        Some((Initialize::DISCRIMINATOR, data)) => {
            Initialize::try_from((accounts, data))?.process()
        }
        Some((Increment::DISCRIMINATOR, _)) => Increment::try_from(accounts)?.process(),
        Some((Decrement::DISCRIMINATOR, _)) => Decrement::try_from(accounts)?.process(),
        Some((Close::DISCRIMINATOR, _)) => Close::try_from(accounts)?.process(),
        _ => Err(ProgramError::InvalidInstructionData),
    }
}
