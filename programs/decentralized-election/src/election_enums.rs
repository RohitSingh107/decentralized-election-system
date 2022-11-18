use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Eq, PartialEq, Clone, Copy, Debug)]
pub enum ElectionPhase {
    Registration,
    Voting,
    Closed,
}

#[error_code]
pub enum ElectionError {
    #[msg("At least 1 winner required")]
    WinnerCountNotAllowed,
    #[msg("Currently not accepting registrations")]
    RegistrationPhaseClosed,
    #[msg("Please ensure only valid candidate can register")]
    WrongPubKey,
    #[msg("You don't have permission to perform this operation")]
    PrivilageNotAllowed,
    #[msg("It is not voting phase")]
    NotAtVotingPhase,
    #[msg("Elections are closed")]
    ElectionIsOver
}
