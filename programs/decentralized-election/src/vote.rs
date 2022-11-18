use anchor_lang::prelude::*;

use crate::election_enums::{ElectionError, ElectionPhase};
use crate::initiate_election::ElectionAccount;
use crate::register::CandidateElectionData;

pub fn vote(ctx: Context<Vote>) -> Result<()> {
    let election_account = &mut ctx.accounts.election_account;

    require!(
        election_account.phase == ElectionPhase::Voting,
        ElectionError::NotAtVotingPhase
    );

    let candidate = &mut ctx.accounts.candidate_election_data;

    let voted_to = &mut ctx.accounts.voted_to;

    candidate.votes += 1;
    voted_to.id = candidate.id;

    Ok(())
}

#[derive(Accounts)]
pub struct Vote<'info> {
    #[account(
        init,
        payer = signer,
        space = 8 + 8,
        seeds = [
        b"voter",
        signer.key().as_ref(),
        election_account.key().as_ref(),
        ],
        bump
        )]
    pub voted_to: Account<'info, VotedTo>,
    #[account(mut)]
    pub candidate_election_data: Account<'info, CandidateElectionData>,
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut)]
    pub election_account: Account<'info, ElectionAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct VotedTo {
    pub id: u64,
}
