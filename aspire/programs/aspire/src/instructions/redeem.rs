use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{
        burn, close_account, transfer_checked, Burn, CloseAccount, Mint, TokenAccount,
        TokenInterface, TransferChecked,
    },
};

use crate::error::ErrorCode;
use crate::Escrow;

#[derive(Accounts)]

// make a repay vault to deposit the fund
pub struct Redeem<'info> {
    #[account(mut)]
    pub funder: Signer<'info>,
    #[account(mut)]
    pub maker: SystemAccount<'info>,
    #[account(
        mut,
        associated_token::mint = mint_fund,
        associated_token::authority = funder,
        associated_token::token_program = token_program,
    )]
    pub funder_ata_fund: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = mint_debt,
        associated_token::authority = funder,
        associated_token::token_program = token_program,
    )]
    pub funder_ata_debt: Box<InterfaceAccount<'info, TokenAccount>>,
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
        close = maker,
        has_one = maker,
        has_one = mint_debt,
        has_one = mint_fund,
        seeds=[b"escrow", maker.key().as_ref(), escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.bump
    )]
    pub escrow: Box<Account<'info, Escrow>>,
    #[account(
        mut,
        associated_token::mint = mint_fund,
        associated_token::authority = escrow,
        associated_token::token_program = token_program
    )]
    pub repay_vault: Box<InterfaceAccount<'info, TokenAccount>>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Redeem<'info> {
    // deposit the debt token
    pub fn redeem(&mut self) -> Result<()> {
        //check the repay_vault is exist and have money
        // require!(self.repay_vault.amount > 0, ErrorCode::InsufficientAmount);
        msg!("{} {}", self.funder_ata_debt.amount, "funder");
        msg!("{} {}", self.escrow.max_fund_amount, "escrow record");
        // require!(
        //     self.funder_ata_debt.amount == self.escrow.max_fund_amount,
        //     ErrorCode::InsufficientAmount
        // );

        let signer_seeds: [&[&[u8]]; 1] = [&[
            b"escrow",
            self.maker.to_account_info().key.as_ref(),
            &self.escrow.seed.to_le_bytes()[..],
            &[self.escrow.bump],
        ]];

        let transfer_accounts = TransferChecked {
            from: self.repay_vault.to_account_info(),
            mint: self.mint_fund.to_account_info(),
            to: self.funder_ata_fund.to_account_info(),
            authority: self.escrow.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            transfer_accounts,
            &signer_seeds,
        );

        transfer_checked(cpi_ctx, self.repay_vault.amount, self.mint_fund.decimals)?;

        //close accounts
        let close_accounts = CloseAccount {
            account: self.repay_vault.to_account_info(),
            destination: self.maker.to_account_info(),
            authority: self.escrow.to_account_info(),
        };

        let ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            close_accounts,
            &signer_seeds,
        );

        close_account(ctx)
        //burn debt token
    }
    pub fn burn_token(&mut self) -> Result<()> {
        let cpi_accounts = Burn {
            mint: self.mint_debt.to_account_info(),
            from: self.funder_ata_debt.to_account_info(),
            authority: self.funder.to_account_info(),
        };

        let ctx = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);
        burn(ctx, self.funder_ata_debt.amount)
    }
}
