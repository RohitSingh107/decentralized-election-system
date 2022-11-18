use anchor_lang::prelude::*;

pub mod apply;
pub mod change_phase;
pub mod election_enums;
pub mod initiate_election;
pub mod register;
pub mod vote;

use crate::election_enums::ElectionPhase;
use apply::*;
use change_phase::*;
use initiate_election::*;
use register::*;
use vote::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod decentralized_election {
    use super::*;

    pub fn initiate_election(ctx: Context<InitiateElection>, winners_count: u8) -> Result<()> {
        initiate_election::initiate_election(ctx, winners_count)
    }

    pub fn apply(ctx: Context<Apply>) -> Result<()> {
        apply::apply(ctx)
    }

    pub fn register(ctx: Context<Register>) -> Result<()> {
        register::register(ctx)
    }

    pub fn change_phase(ctx: Context<ChangePhase>, new_phase: ElectionPhase) -> Result<()> {
        change_phase::change_phase(ctx, new_phase)
    }

    pub fn vote(ctx: Context<Vote>) -> Result<()> {
        vote::vote(ctx)
    }
}
