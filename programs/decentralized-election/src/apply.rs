use anchor_lang::prelude::*;

use crate::election_enums::{ElectionError, ElectionPhase};
use crate::initiate_election::ElectionAccount;


pub fn apply(ctx: Context<Apply>) -> Result<()> {

    let election_account = &mut ctx.accounts.election_account;

    require!(election_account.phase == ElectionPhase::Registration, ElectionError::RegistrationPhaseClosed);

    election_account.candidates_count += 1;
    ctx.accounts.candidate_id.id = election_account.candidates_count;
    ctx.accounts.candidate_id.pubkey = ctx.accounts.signer.key();


    Ok(())
    
}

#[derive(Accounts)]
pub struct Apply<'info> {
    
    #[account(
        init,
        payer= signer,
        space = 8 + 8 + 32,
        seeds = [

        b"candidate",
        signer.key.as_ref()

        ],
        bump
        )]
    pub candidate_id : Account<'info, CandidateID>,

    #[account(mut)]
    pub election_account : Account<'info, ElectionAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,

}

#[account]
pub struct CandidateID{
    pub id: u64,
    pub pubkey: Pubkey,
} 
