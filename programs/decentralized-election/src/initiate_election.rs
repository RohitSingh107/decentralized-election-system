use anchor_lang::prelude::*;

use crate::election_enums::{ElectionError, ElectionPhase};

pub fn initiate_election(ctx: Context<InitiateElection>, winners_count: u8) -> Result<()> {
    require!(winners_count > 0, ElectionError::WinnerCountNotAllowed);
    let election_account = &mut ctx.accounts.election_account;
    election_account.candidates_count = 0;
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
    pub candidates_count: u64,
    pub phase: ElectionPhase,
    pub election_officer: Pubkey,
    pub winners_count: u8,
    pub winners_id: Vec<u64>,
    pub winners_votes_count: Vec<u64>,
}

impl ElectionAccount {
    pub fn close_registration(&mut self) -> Result<()> {
        require!(
            self.phase == ElectionPhase::Registration,
            ElectionError::RegistrationPhaseClosed
        );

        if self.candidates_count <= self.winners_count as u64 {
            for i in 1..=self.candidates_count {
                self.winners_id.push(i);
                self.phase = ElectionPhase::Closed;
            }
        } else {
            self.phase = ElectionPhase::Voting;
        }
        Ok(())
    }

    pub fn close_voting(&mut self) -> Result<()> {
        require!(
            self.phase == ElectionPhase::Voting,
            ElectionError::NotAtVotingPhase
        );
        self.phase = ElectionPhase::Closed;
        Ok(())
    }


    pub fn record_vote(&mut self, id: u64, votes: u64){

        if !self.winners_id.contains(&id) {
            if self.winners_id.len() < self.winners_count as usize {
                self.winners_id.push(id);
                self.winners_votes_count.push(votes);
            } else {
                let current_last_winner = (self.winners_count -1) as usize;

                if votes > self.winners_id[current_last_winner] {
                    self.winners_votes_count[current_last_winner] = votes;
                } else {
                    return;
                }
            }
        }

        // Sorting Votes
        let mut p = self.winners_id.iter().position(|&r| r == id).unwrap();

        while p > 0 && self.winners_votes_count[p] > self.winners_votes_count[p-1] {
            let vote_holder = self.winners_votes_count[p-1];
            let id_holder = self.winners_id[p-1];

            self.winners_votes_count[p-1] = self.winners_votes_count[p];
            self.winners_votes_count[p] = vote_holder;

            self.winners_id[p-1] = self.winners_id[p];
            self.winners_id[p] = id_holder;

            p -= 1;
        }

    }


}
