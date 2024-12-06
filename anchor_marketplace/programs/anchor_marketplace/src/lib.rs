pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("AHgDYBk7KE7Vw4hrvP3V7SFP1rtCBcSvdG3kXGHCYV1t");

#[program]
pub mod anchor_marketplace {

    use super::*;

    // pub fn initialize(ctx: Context<Initialize>, name: str, fee: u16) -> Result<()> {
    //     ctx.accounts.init(name, fee, ctx.bumps)
    // }
    // pub fn list(ctx: Context<Listing>) -> Result<()> {
    //     ctx.accounts.create_listing(price, &ctx.bumps);
    //     ctx.accounts.deposit_nft()
    // }
    // pub fn purchase(ctx: Context<Purchase>) -> Result<()> {
    //     ctx.accounts.send_sol();
    //     ctx.accounts.send_nft();
    //     ctx.accounts.close_mint_vault()
    // }
    // pub fn delist(ctx: Context<Delisting>) -> Result<()> {
    //     ctx.accounts.withdraw_nft();
    //     ctx.accounts.close_mint_vault()
    // }
}
