use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Escrow {
    pub seed: u64,
    pub maker: Pubkey,
    pub mint_debt: Pubkey,
    pub mint_fund: Pubkey,
    pub max_fund_amount: u64,
    pub total_funds: u64,
    pub total_repay: u64,
    pub interest_rate_per_year: u64,
    pub borrow_start: i64,
    pub bump: u8
}