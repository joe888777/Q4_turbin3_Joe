use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{close_account, CloseAccount, transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};

use crate::Escrow;

// get the debt token and fund the maker

#[derive(Accounts)]
pub struct Fund<'info> {
    #[account(mut)]
    pub funder: Signer<'info>,
    #[account(mut)]
    pub maker: SystemAccount<'info>,
    pub mint_debt: InterfaceAccount<'info, Mint>,
    pub mint_fund: InterfaceAccount<'info, Mint>,
    #[account(
        init_if_needed,
        payer = funder,
        associated_token::mint = mint_debt,
        associated_token::authority = funder,
        associated_token::token_program = token_program,
    )]
    pub funder_ata_debt: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = mint_fund,
        associated_token::authority = funder,
        associated_token::token_program = token_program,
    )]
    pub funder_ata_fund: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        init_if_needed,
        payer=funder,
        associated_token::mint = mint_fund,
        associated_token::token_program = token_program,
        associated_token::authority = maker
    )]
    pub maker_ata_fund: Box<InterfaceAccount<'info, TokenAccount>>,
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
        mut,
        associated_token::mint = mint_debt,
        associated_token::authority = escrow,
        associated_token::token_program = token_program
    )]
    pub vault: Box<InterfaceAccount<'info, TokenAccount>>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Fund<'info> {
    pub fn deposit(&mut self) -> Result<()> {
        let transfer_accounts = TransferChecked {
            from: self.funder_ata_fund.to_account_info(),
            to: self.maker_ata_fund.to_account_info(),
            mint: self.mint_fund.to_account_info(),
            authority: self.funder.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), transfer_accounts);
        transfer_checked(
            cpi_ctx,
            self.escrow.max_fund_amount,
            self.mint_fund.decimals,
        )?;

        self.escrow.borrow_start = Clock::get()?.unix_timestamp;
        self.escrow.total_funds = self.escrow.max_fund_amount;

        Ok(())
    }

    pub fn withdraw_and_close_account(&mut self) -> Result<()> {
        let signer_seeds: [&[&[u8]]; 1] = [&[
            b"escrow",
            self.maker.to_account_info().key.as_ref(),
            &self.escrow.seed.to_le_bytes()[..],
            &[self.escrow.bump]
        ]];

        let transfer_accounts = TransferChecked {
            from: self.vault.to_account_info(),
            to: self.funder_ata_debt.to_account_info(), 
            mint: self.mint_debt.to_account_info(),
            authority: self.escrow.to_account_info()
        };

        let ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            transfer_accounts,
            &signer_seeds
        );
        transfer_checked(ctx, self.escrow.max_fund_amount, self.mint_debt.decimals)?;

        let close_accounts = CloseAccount {
            account: self.vault.to_account_info(),
            destination: self.maker.to_account_info(),
            authority: self.escrow.to_account_info()
        };

        let ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            close_accounts,
            &signer_seeds
        );

        close_account(ctx)
    }
}
