use anchor_lang::prelude::*;
use crate::constants::*;

#[account]
pub struct Config {
    pub mint_x: Pubkey,
    pub mint_y: Pubkey,
    pub authority: Option<Pubkey>,
    pub seed: u64,
    pub fee: u16,
    pub locked: bool,
    pub auth_bump: u8,
    pub config_bump: u8,
}

impl Config {
    pub const LEN: usize = 8 + PUBKEY_L + U64_L + U16_L + BOOL_L + U8_L*2;
    pub fn init(
        &mut self,
        seed: u64,
        authority: Option<Pubkey>, 
        mint_x: Pubkey, 
        mint_y: Pubkey,
        fee: u16,
        locked: bool,
        auth_bump: u8,
        config_bump: u8
    ) {
        self.seed = seed;
        self.authority = authority;
        self.mint_x = mint_x;
        self.mint_y = mint_y;
        self.fee = fee;
        self.locked = locked;
        self.auth_bump = auth_bump;
        self.config_bump = config_bump;
    }
}