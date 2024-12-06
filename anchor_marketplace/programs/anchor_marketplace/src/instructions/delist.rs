use anchor_lang::prelude::*;
use anchor_spl::{
    token_interface::{
        close_account, CloseAccount, Mint, TokenAccount, TokenInterface, transfer_checked, TransferChecked},
};

use crate::state::{Marketplace, Listing};

#[derive(Accounts)]
pub struct Delisting <'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    #[account(
        seeds=[b"marketplace", marketplace.name.as_str().as_bytes()],
        bump = marketplace.bump
    )]
    pub marketplace: Account<'info, Marketplace>,
    pub maker_mint: InferfaceAccount<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = maker_mint,
        associated_token::authority = maker,
    )]
    pub maker_ata: InferfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = maker_mint,
        associated_token::authority = maker,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        close = maker,
        has_one = maker,
        seeds = [marketplace.key().as_ref(), maker_mint.key().as_ref()],
        bump = listing.bump,
    )]
    pub listing: Account<'info, Listing>,
    pub token_program: Program<'info, TokenInterface>,
    pub system_program: Program<'info, System>
}

impl <'info> Delisting <'info> {
    pub fn withdraw_nft(&mut self) -> Result<()> {
        let signer_seeds = &[
            &self.marketplace.key().to_bytes()[..],
            &self.maker_mint.key().to_bytes()[..],
            &[self.listing.bump]
        ];
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = TransferChecked {
            from: self.vault.to_account_info(),
            mint: self.maker_mint.to_account_info(),
            to: self.maker_ata.to_account_info(),
            authority: self.listing.to_account_info()
        };
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, &signer_seeds);
        transfer_checked(cpi_ctx, 1, self.maker_mint.decimal)
    }

    pub fn close_mint_vault(&mut self) -> Result<()> {
        let signer_seeds = &[
            &self.marketplace.key().to_bytes()[..],
            &self.maker_mint.key().to_bytes()[..],
            &[self.listing.bump]
        ];
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = CloseAccount{
            account: self.vault.to_account_info(),
            destination: self.maker.to_account_info(),
            authority: self.listing.to_account_info()
        };
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, &signer_seeds);
        close_account(ctx)
    }
}