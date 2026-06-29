use pinocchio::{error::ProgramError, AccountView, Address, ProgramResult};

use crate::{constants::SEED, error::CounterError, state::Counter};

pub struct DecrementAccounts<'a> {
    counter: &'a mut AccountView,
}
impl<'a> TryFrom<&'a mut [AccountView]> for DecrementAccounts<'a> {
    type Error = ProgramError;
    fn try_from(accounts: &'a mut [AccountView]) -> Result<Self, Self::Error> {
        let [user, counter] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };
        if !user.is_signer() {
            return Err(ProgramError::MissingRequiredSignature);
        }
        if !counter.owned_by(&crate::ID) {
            return Err(ProgramError::InvalidAccountOwner);
        }
        let counter_pda = {
            let counter_data = counter.try_borrow()?;
            let counter_bump = Counter::from_bytes(counter_data.as_ref())?.bump();
            Address::derive_address(
                &[SEED.as_bytes(), user.address().as_ref()],
                Some(counter_bump),
                &crate::ID,
            )
        };
        if *counter.address() != counter_pda {
            return Err(CounterError::InvalidCounterAccount.into());
        }
        Ok(Self { counter })
    }
}

pub struct Decrement<'a> {
    accounts: DecrementAccounts<'a>,
}
impl<'a> TryFrom<&'a mut [AccountView]> for Decrement<'a> {
    type Error = ProgramError;
    fn try_from(accounts: &'a mut [AccountView]) -> Result<Self, Self::Error> {
        let accounts = DecrementAccounts::try_from(accounts)?;
        Ok(Self { accounts })
    }
}
impl<'a> Decrement<'a> {
    pub const DISCRIMINATOR: &'a u8 = &2u8;
    pub fn process(&mut self) -> ProgramResult {
        let mut data = self.accounts.counter.try_borrow_mut()?;
        let counter = Counter::from_bytes_mut(data.as_mut())?;
        counter.set_count(
            counter
                .count()
                .checked_sub(1)
                .ok_or(CounterError::DecrementUnderflow)?,
        );
        Ok(())
    }
}
