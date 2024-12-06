use anchor_lang::prelude::*;
use anchor_spl::{
    token::{
        approve, close_account, Approve, CloseAccount, Mint, Token, TokenAccount
    }, token_2022::spl_token_2022::solana_zk_token_sdk::instruction::transfer, token_interface::{transfer_checked, TokenInterface, TransferChecked}
};

use crate::{marketplace, state::{Listing, Marketplace}};
use crate::error::ErrorCode::NameTooLong;

#[derive(Accounts)]
pub struct Purchase <'info> {
    pub taker: Signer<'info>,
    #[account(
        mut,

    )]
    pub maker: Account<'info>,
    pub marketplace: Account<'info, Marketplace>,
    pub maker_mint: InferfaceAccount<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = maker_mint,
        associated_token::authority = maker,
    )]
    pub maker_ata: InferfaceAccount<'info, TokenAccount>,
    #[account(
        init,
        payer = maker,
        associated_token::mint = maker_mint,
        associated_token::authority = maker,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init,
        payer = maker,
        seeds = [marketplace.key().as_ref(), maker_mint.key().as_ref()],
        bump,
        space = Listing::INIT_SPACE
    )]
    pub listing: Account<'info, Listing>,
    pub collection_mint: InterfaceAccount<'info, Mint>,
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = maker_mint,
        associated_token::authority = maker,
    )]
    pub taker_ata: InterfaceAccount<'info, TokenAccount>,
    
    #[account(
        seeds = [b"treasury", marketplace.key().as_ref()],
        bump
    )]
    pub treasury: SystemAccount<'info>,
    #[account(
        init,
        payer = admin,
        seeds = [b"rewards", marketplace.key().as_ref()],
        bump = marketplace.reward_bump,
        mint::authority = marketplace
    )]
    pub rewards_mint: InferfaceAccount<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, TokenInterface>
}

impl <'info> Purchase <'info> {
    pub fn send_sol (&mut self) -> Result<()> {
        let cpi_program = self.system_program.to_account_info();
        let cpi_accounts = Trasfer {
            from: self.taker.to_account_info(),
            to: self.maker.to_account_info()
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        let amount = self.listing.price.checked_sub(self.marketplace.fee as u64).unwrap();
        trasfer(cpi_ctx, amount);

        let cpi_program = self.system_program.to_account_info();
        let cpi_accounts = Transfer {
            from: self.taker.to_account_info(),
            to: self.treasury.to_account_info()
        };
        let cpi_ctx = CpiContext::new (
            cpi_program, cpi_accounts
        );
        let marketplace_fee = (self.marketplace.fee as u64)
            .checked_div(10000_u64).unwrap()
            .checked_mul(self.listing.price as u64);
        trasnfer(cpi_ctx, marketplace_fee)
    }
    pub fn send_nft (&mut self) -> Result<()> {
        let signer_seeds = &[
            &self.marketplace.key().to_bytes()[..],
            &self.maker_mint.key().to_bytes()[..],
            &[self.listing.bump]
        ];
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = TransferChecked {
            from: self.vault.to_account_info(),
            mint: self.maker_mint.to_account_info(),
            to: self.taker_ata.to_account_info(),
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