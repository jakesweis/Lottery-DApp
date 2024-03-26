use anchor_lang::prelude::*;

pub const MASTER_SEED: &str = "MASTER_SEED";
pub const LOTTERY_SEED: &str = "LOTTERY_SEED";
pub const TICKET_SEED: &str = "TICKET_SEED";

#[account]
pub struct Master {
    pub last_id:u32, 
}

#[account]
pub struct Lottery {
    pub id: u32,
    pub authority: Pubkey,
    pub ticket_price: u64,
    pub last_ticket_id: u32,
    pub winner_id: Option<u32>,
    pub claimed: bool,
}

#[account]
pub struct Ticket {
    pub id: u32,
    pub authority: Pubkey,
    pub lottery_id: u32,
}
