use anchor_lang::prelude::*;

use crate::apply::CandidateID;
use crate::election_enums::ElectionError;
use crate::initiate_election::ElectionAccount;

pub fn register(ctx: Context<Register>) -> Result<()> {
    let candidate = &mut ctx.accounts.candidate_election_data;

    candidate.votes = 0;
    candidate.pubkey = ctx.accounts.signer.key();
    candidate.id = ctx.accounts.candidate_id.id;

    Ok(())
}

#[derive(Accounts)]
pub struct Register<'info> {
    #[account(
        init,
        payer = signer,
        space = 8 + 8 + 32,
    seeds = [
        b"candidate",
        signer.key().as_ref(),

        ],
        bump
        )]
    pub candidate_election_data: Account<'info, CandidateElectionData>,
    pub election_account: Account<'info, ElectionAccount>,
    pub candidate_id: Account<'info, CandidateID>,
    #[account(mut, address=candidate_id.pubkey @ ElectionError::WrongPubKey )]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct CandidateElectionData {
    pub votes: u64,
    pub id: u64,
    pub pubkey: Pubkey,
}

// why are we using two endpoints? Solana requires the transaction to include all the accounts being read or written. We don't know the ID of the candidate before calling the apply() endpoint, so we can't generate the PDA of the 'CandidateData' account before the 'CandidateIdentity' account is created. So, the apply() endpoint first creates the ‘CandidateIdentity’ account whose fields are used to create the ‘CandidateData’ account in the register() endpoint.
