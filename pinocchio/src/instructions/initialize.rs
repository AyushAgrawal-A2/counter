use pinocchio::{
    cpi::{Seed, Signer},
    error::ProgramError,
    sysvars::{rent::Rent, Sysvar},
    AccountView, Address, ProgramResult,
};
use pinocchio_system::instructions::CreateAccount;

use crate::{constants::SEED, error::CounterError, state::Counter};

pub struct InitializeAccounts<'a> {
    user: &'a AccountView,
    counter: &'a mut AccountView,
    counter_bump: u8,
}
impl<'a> TryFrom<&'a mut [AccountView]> for InitializeAccounts<'a> {
    type Error = ProgramError;
    fn try_from(accounts: &'a mut [AccountView]) -> Result<Self, Self::Error> {
        let [user, counter, _system_program] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };
        if !user.is_signer() {
            return Err(ProgramError::MissingRequiredSignature);
        }
        if !counter.owned_by(&crate::ID) {
            return Err(ProgramError::InvalidAccountOwner);
        }
        let (counter_pda, counter_bump) = Address::derive_program_address(
            &[SEED.as_bytes(), user.address().as_ref()],
            &crate::ID,
        )
        .ok_or(ProgramError::InvalidSeeds)?;
        if *counter.address() != counter_pda {
            return Err(CounterError::InvalidCounterAccount.into());
        }
        Ok(Self {
            user,
            counter,
            counter_bump,
        })
    }
}

pub struct InitializeInstructionData {
    count: u64,
}
impl<'a> TryFrom<&[u8]> for InitializeInstructionData {
    type Error = ProgramError;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        if data.len() != 8 {
            return Err(ProgramError::InvalidInstructionData);
        }
        let count = u64::from_le_bytes(data.try_into().unwrap());
        Ok(Self { count })
    }
}

pub struct Initialize<'a> {
    accounts: InitializeAccounts<'a>,
    instruction_data: InitializeInstructionData,
}
impl<'a> TryFrom<(&'a mut [AccountView], &[u8])> for Initialize<'a> {
    type Error = ProgramError;
    fn try_from((accounts, data): (&'a mut [AccountView], &[u8])) -> Result<Self, Self::Error> {
        let accounts = InitializeAccounts::try_from(accounts)?;
        let instruction_data = InitializeInstructionData::try_from(data)?;
        Ok(Self {
            accounts,
            instruction_data,
        })
    }
}
impl<'a> Initialize<'a> {
    pub const DISCRIMINATOR: &'a u8 = &0u8;
    pub fn process(&mut self) -> ProgramResult {
        let lamports = Rent::get()?.minimum_balance_unchecked(Counter::LEN);
        let bump = [self.accounts.counter_bump];
        let seeds = [
            Seed::from(SEED.as_bytes()),
            Seed::from(self.accounts.user.address().as_ref()),
            Seed::from(&bump),
        ];
        let signers = [Signer::from(&seeds)];
        CreateAccount {
            from: self.accounts.user,
            to: self.accounts.counter,
            lamports,
            space: Counter::LEN as u64,
            owner: &crate::ID,
        }
        .invoke_signed(&signers)?;
        let mut data = self.accounts.counter.try_borrow_mut()?;
        let counter = Counter::from_bytes_mut(data.as_mut())?;
        counter.set_count(self.instruction_data.count);
        counter.set_bump(self.accounts.counter_bump);
        Ok(())
    }
}
