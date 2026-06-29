use pinocchio::error::ProgramError;

#[repr(C)]
pub struct Counter {
    count: [u8; 8],
    bump: [u8; 1],
}
impl Counter {
    pub const LEN: usize = core::mem::size_of::<Self>();
    #[inline(always)]
    pub fn from_bytes(data: &[u8]) -> Result<&Self, ProgramError> {
        if data.len() != Self::LEN {
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(unsafe { &*(data.as_ptr() as *const Self) })
    }
    #[inline(always)]
    pub fn from_bytes_mut(data: &mut [u8]) -> Result<&mut Self, ProgramError> {
        if data.len() != Self::LEN {
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(unsafe { &mut *(data.as_mut_ptr() as *mut Self) })
    }
    #[inline(always)]
    pub fn count(&self) -> u64 {
        u64::from_le_bytes(self.count)
    }
    #[inline(always)]
    pub fn set_count(&mut self, count: u64) {
        self.count = count.to_le_bytes();
    }
    #[inline(always)]
    pub fn bump(&self) -> u8 {
        self.bump[0]
    }
    #[inline(always)]
    pub fn set_bump(&mut self, bump: u8) {
        self.bump = [bump];
    }
}
