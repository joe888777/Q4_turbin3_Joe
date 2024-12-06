use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};

use crate::state::Escrow;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Make<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
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
        associated_token::mint = mint_debt,
        associated_token::authority = maker,
        associated_token::token_program = token_program,
    )]
    pub maker_ata_debt: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        init,
        payer=maker,
        seeds = [b"escrow", maker.key().as_ref(), seed.to_le_bytes().as_ref()],
        space = 8 + Escrow::INIT_SPACE,
        bump
    )]
    pub escrow: Box<Account<'info, Escrow>>,

    #[account(
        init_if_needed,
        payer=maker,
        associated_token::mint = mint_debt,
        associated_token::authority = escrow,
        associated_token::token_program = token_program
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Make<'info> {
    pub fn initialize(
        &mut self,
        seed: u64,
        max_fund_amount: u64,
        interest_rate_per_year: u64,
        bumps: &MakeBumps,
    ) -> Result<()> {
        self.escrow.set_inner(Escrow {
            seed,
            maker: self.maker.key(),
            mint_debt: self.mint_debt.key(),
            mint_fund: self.mint_fund.key(),
            total_funds: 0,
            total_repay: 0,
            interest_rate_per_year,
            borrow_start: 0,
            max_fund_amount,
            bump: bumps.escrow,
        });

        Ok(())
    }

    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        let transfer_accounts = TransferChecked {
            from: self.maker_ata_debt.to_account_info(),
            mint: self.mint_debt.to_account_info(),
            to: self.vault.to_account_info(),
            authority: self.maker.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), transfer_accounts);

        transfer_checked(cpi_ctx, amount, self.mint_debt.decimals)
    }
}
