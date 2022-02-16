use anchor_lang::prelude::*;

#[error]
pub enum ErrorCode {
    #[msg("Invalid Instruction")]
    InvalidInstruction,
    #[msg("Not Rent Exempt")]
    NotRentExempt,
    #[msg("Invalid Timestamp")]
    InvalidTimestamp,
    #[msg("Incorrect Program Id")]
    IncorrectProgramId,
    #[msg("The seed provided for the PDA account is too long")]
    SeedTooLong,
    #[msg("Debug Error")]
    DebugError,
}

// Implements a custom escrow error
// impl From<ErrorCode> for ProgramError {
//     fn from(e:ErrorCode) -> Self {
//         ProgramError::Custom(e as u32)
//     }
// }