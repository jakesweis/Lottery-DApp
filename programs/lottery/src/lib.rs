use anchor_lang::prelude::*;
use crate::instructions::*;

pub mod instructions;
pub mod states;
pub mod errors;

declare_id!("HvJymJvJKu1REDZ4Jr6HrzSDNMpTZXsh48PGmjAGa3TS");

#[program]
pub mod lottery {
    use super::*;

    pub fn initialize(ctx: Context<InitializeMaster>) -> Result<()> {
        initialize_master(ctx)
    }

    pub fn new_lottery(ctx: Context<CreateLotteryContext>, ticket_price: u64) -> Result<()> {
        create_lottery(ctx, ticket_price)
    }

    pub fn purchase_ticket(ctx: Context<BuyTicketContext>, lottery_id: u32) -> Result<()> {
        buy_ticket(ctx, lottery_id)
    }

    pub fn choose_winner(ctx: Context<PickWinnerContext>, lottery_id: u32) -> Result<()> {
        pick_winner(ctx, lottery_id)
    }

    pub fn prize_claim(ctx: Context<ClaimPrizeContext>, lottery_id: u32, ticket_id: u32) -> Result<()> {
        claim_prize(ctx, lottery_id, ticket_id)
    }
}