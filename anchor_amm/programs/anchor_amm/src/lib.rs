pub mod constants;
pub mod error;
pub mod helpers;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("Ao68GvmXEvPtFBjfgYCWP3XPyHsPRma3d3kRXvgDzYpj");

#[program]
pub mod anchor_amm {

    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        seed: u64,
        fee: u16,
        authority: Option<Pubkey>,
    ) -> Result<()> {
        ctx.accounts.init(&ctx.bumps, seed, fee, authority)
    }

    pub fn deposit(
        ctx: Context<Deposit>,
        amount: u64,
        max_x: u64,
        max_y: u64,
        expiration: i64,
    ) -> Result<()> {
        ctx.accounts.deposit(amount, max_x, max_y, expiration)
    }

    pub fn withdraw(
        ctx: Context<Withdraw>,
        amount: u64,
        min_x: u64,
        min_y: u64,
        expiration: i64,
    ) -> Result<()> {
        ctx.accounts.withdraw(amount, min_x, min_y, expiration)
    }

    pub fn swap(
        ctx: Context<Swap>,
        is_x: bool,
        amount: u64,
        min: u64,
        expiration: i64,
    ) -> Result<()> {
        ctx.accounts.swap(is_x, amount, min, expiration)
    }

}
