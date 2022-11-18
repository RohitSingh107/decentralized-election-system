use anchor_lang::prelude::*;

use crate::election_enums::{ElectionError, ElectionPhase};
use crate::initiate_election::ElectionAccount;



pub fn change_phase(ctx: Context<ChangePhase>, new_phase: ElectionPhase) -> Result<()> {
    let election_account = &mut ctx.accounts.election_account;

    require!(election_account.phase != ElectionPhase::Closed, ElectionError::ElectionIsOver);

    match new_phase {
        ElectionPhase::Voting => return election_account.close_registration(),
        ElectionPhase::Closed => return election_account.close_voting(),
        ElectionPhase::Registration => return Err(ElectionError::PrivilageNotAllowed.into())
    }
}


#[derive(Accounts)]
pub struct ChangePhase<'info> {
    #[account(mut)]
    pub election_account : Account<'info, ElectionAccount>,
    #[account(mut, address = election_account.election_officer @ ElectionError::PrivilageNotAllowed)]
    pub signer: Signer<'info>,
}

