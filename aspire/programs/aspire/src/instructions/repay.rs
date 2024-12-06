use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};

use crate::Escrow;
use crate::error::ErrorCode;

#[derive(Accounts)]

// make a repay vault to deposit the fund
pub struct Repay<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    #[account(
        init_if_needed,
        payer = maker,
        associated_token::mint = mint_fund,
        associated_token::authority = maker,
        associated_token::token_program = token_program,
    )]
    pub maker_ata_fund: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        mint::token_program = token_program
    )]
    pub mint_debt: InterfaceAccount<'info, Mint>,
    #[account(
        mint::token_program = token_program
    )]
    pub mint_fund: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        has_one = maker,
        has_one = mint_debt,
        has_one = mint_fund,
        seeds=[b"escrow", maker.key().as_ref(), escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.bump
    )]
    pub escrow: Box<Account<'info, Escrow>>,
    #[account(
        init,
        payer=maker,
        associated_token::mint = mint_fund,
        associated_token::authority = escrow,
        associated_token::token_program = token_program
    )]
    pub repay_vault: Box<InterfaceAccount<'info, TokenAccount>>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Repay<'info> {
    // deposit the debt token
    pub fn repay(&mut self) -> Result<()> {
        let duration: i64 = Clock::get()?.unix_timestamp - self.escrow.borrow_start;
        // fund * (1 + intrest_rate * period)
        let repay_amount = self.escrow.max_fund_amount
        * (1_i64 + duration / (86400 * 365)) as u64
        * self.escrow.interest_rate_per_year as u64;

        require!(self.maker_ata_fund.amount > repay_amount, ErrorCode::InsufficientAmount);

        let transfer_accounts = TransferChecked {
            from: self.maker_ata_fund.to_account_info(),
            mint: self.mint_fund.to_account_info(),
            to: self.repay_vault.to_account_info(),
            authority: self.maker.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), transfer_accounts);

        transfer_checked(cpi_ctx, repay_amount, self.mint_fund.decimals)?;
        self.escrow.total_repay = self.escrow.max_fund_amount;
        Ok(())
    }

}
