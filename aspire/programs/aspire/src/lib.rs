pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("66vaBdJJQXqxdKTvpzRM6AnYDAZesSLvR7vVUo5fzBLz");

#[program]
pub mod aspire {
    use super::*;

    pub fn initialize(
        ctx: Context<Make>,
        seed: u64,
        interest_rate_per_year: u64,
        max_fund_amount: u64,
    ) -> Result<()> {
        ctx.accounts
            .initialize(seed, max_fund_amount, interest_rate_per_year, &ctx.bumps)?;
        ctx.accounts.deposit(max_fund_amount)?;
        Ok(())
    }

    pub fn fund(ctx: Context<Fund>) -> Result<()> {
        ctx.accounts.deposit()?;
        ctx.accounts.withdraw_and_close_account()
    }

    pub fn repay(ctx: Context<Repay>) -> Result<()> {
        ctx.accounts.repay()
    }
    pub fn redeem(ctx: Context<Redeem>) -> Result<()> {
        // ctx.accounts.burn_token()?;
        ctx.accounts.redeem()
    }
}
