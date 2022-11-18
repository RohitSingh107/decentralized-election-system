use anchor_lang::prelude::*;

#[derive(AnchorDeserialize, AnchorSerialize, PartialEq, Eq, Clone)]
pub enum ElectionPhase {
    Registration,
    Voting,
    Closed,
}

#[error_code]
pub enum ElectionError {
    WinnerCountNotAllowed,
    RegistrationPhaseClosed,
}
