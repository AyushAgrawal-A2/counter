use pinocchio::{error::ProgramError, AccountView, Address, ProgramResult};

use crate::{constants::SEED, error::CounterError, state::Counter};

pub struct CloseAccounts<'a> {
    user: &'a mut AccountView,
    counter: &'a mut AccountView,
}
impl<'a> TryFrom<&'a mut [AccountView]> for CloseAccounts<'a> {
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
        Ok(Self { user, counter })
    }
}

pub struct Close<'a> {
    accounts: CloseAccounts<'a>,
}
impl<'a> TryFrom<&'a mut [AccountView]> for Close<'a> {
    type Error = ProgramError;
    fn try_from(accounts: &'a mut [AccountView]) -> Result<Self, Self::Error> {
        let accounts = CloseAccounts::try_from(accounts)?;
        Ok(Self { accounts })
    }
}
impl<'a> Close<'a> {
    pub const DISCRIMINATOR: &'a u8 = &3u8;
    pub fn process(&mut self) -> ProgramResult {
        self.accounts.user.set_lamports(
            self.accounts
                .user
                .lamports()
                .checked_add(self.accounts.counter.lamports())
                .ok_or(CounterError::LamportsOverflow)?,
        );
        self.accounts.counter.close()?;
        Ok(())
    }
}
