use anchor_lang::prelude::*;

use crate::election_enums::{ElectionError, ElectionPhase};

pub fn initiate_election(ctx: Context<InitiateElection>, winners_count: u8) -> Result<()> {
    require!(winners_count > 0, ElectionError::WinnerCountNotAllowed);
    let election_account = &mut ctx.accounts.election_account;
    election_account.candidate = 0;
    election_account.phase = ElectionPhase::Registration;
    election_account.election_officer = ctx.accounts.signer.key();
    election_account.winners_count = winners_count;
    Ok(())
}
#[derive(Accounts)]
#[instruction(winners_count:u8)]
pub struct InitiateElection<'info> {
    #[account(
        init,
        payer = signer,
        space = 8 + 8 + 2 + 32 + 1 + 2 * (4 + winners_count as usize * 8)
        )]
    pub election_account: Account<'info, ElectionAccount>,

    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct ElectionAccount {
    pub candidate: u64,
    pub phase: ElectionPhase,
    pub election_officer: Pubkey,
    pub winners_count: u8,
    pub winners_id: Vec<u64>,
    pub winners_votes_count: Vec<u64>,
}
