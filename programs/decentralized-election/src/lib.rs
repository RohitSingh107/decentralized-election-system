
use anchor_lang::prelude::*;

pub mod election_enums;
pub mod initiate_election;
pub mod apply;

use initiate_election::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod decentralized_election {
    use super::*;

    pub fn initiate_election(ctx: Context<InitiateElection>, winners_count: u8) -> Result<()> {
        initiate_election::initiate_election(ctx, winners_count)
    }
}
