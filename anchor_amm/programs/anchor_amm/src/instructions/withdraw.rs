use anchor_lang::prelude::*;

use crate::state::config::Config;
use crate::error::AmmError;
use crate::{assert_non_zero, assert_not_expired, assert_not_locked};
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{
        mint_to,
        transfer_checked,
        Mint,
        MintTo,
        TokenAccount,
        TokenInterface,
        TransferChecked,
        burn,
        Burn
    }
};

use constant_product_curve::{ConstantProduct, LiquidityPair, XYAmounts};

#[derive(Accounts)]
pub struct Withdraw <'info> {
    #[account(mut)]
    user: Signer<'info>,
    pub mint_x: Box<InterfaceAccount<'info, Mint>>,
    pub mint_y: Box<InterfaceAccount<'info, Mint>>,
    #[account(
        mut,
        associated_token::mint = mint_x,
        associated_token::authority = auth
    )]
    pub vault_x: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = mint_y,
        associated_token::authority = auth
    )]
    pub vault_y: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        seeds = [b"auth"],
        bump = config.auth_bump
    )]
    pub auth: UncheckedAccount<'info>,
    #[account(
        init_if_needed,
        payer = user,
        seeds = [b"lp", config.key().as_ref()],
        bump,
        mint::decimals = 6,
        mint::authority = auth
    )]
    pub mint_lp: Box<InterfaceAccount<'info, Mint>>,
    #[account(
        mut,
        associated_token::mint = mint_x,
        associated_token::authority = user
    )]
    pub user_x: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = mint_y,
        associated_token::authority = user
    )]
    pub user_y: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = mint_lp,
        associated_token::authority = user
    )]
    pub user_lp: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        mut,
        seeds = [b"config", config.seed.to_le_bytes().as_ref()],
        bump = config.config_bump,
    )]
    pub config: Account<'info, Config>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>

}

impl <'info> Withdraw <'info> {
    pub fn withdraw(
        &mut self,
        amount: u64,
        min_x: u64,
        min_y: u64,
        expiration: i64
    ) -> Result<()> {
        assert_not_locked!(self.config.locked);
        assert_not_expired!(expiration);
        assert_non_zero!([amount]);
        let amounts: XYAmounts = ConstantProduct::xy_withdraw_amounts_from_l(
            self.vault_x.amount,
            self.vault_y.amount,
            self.mint_lp.supply,
            amount,
            6
        )
        .map_err(AmmError::from)?;
        require!(
            min_x <= amounts.x && min_y <= amounts.y,
            AmmError::SlippageExceeded
        );
        self.withdraw_token(true, amounts.x);
        self.withdraw_token(false, amounts.y);
        self.burn_lp_tokens(amount)
    }

    pub fn withdraw_token(&mut self, is_x: bool, amount: u64) -> Result<()> {
        let mint;
        let (from, to) = match is_x {
            true => {

                mint = self.mint_x.clone();
                (
                    self.vault_x.to_account_info(),
                    self.user_x.to_account_info(),
                )
            },
            false => {
                mint = self.mint_y.clone();
                (
                    self.vault_y.to_account_info(),
                    self.user_y.to_account_info(),
                )
            },
        };
        let seeds = &[
            &b"auth"[..],
            &[self.config.auth_bump]
        ];
        let signer_seeds = &[&seeds[..]];
        let cpi_account = TransferChecked {
            from: from,
            mint: mint.to_account_info(),
            to: to,
            authority: self.user.to_account_info(),
        };
        let ctx = CpiContext::new_with_signer(self.token_program.to_account_info(), cpi_account, signer_seeds);
        transfer_checked(ctx, amount, 6)
    }
    pub fn burn_lp_tokens(&mut self, amount: u64) -> Result<()> {
        let accounts = Burn {
            mint: self.mint_lp.to_account_info(),
            from: self.user_lp.to_account_info(),
            authority: self.user.to_account_info()
        };
        let ctx = CpiContext::new(self.token_program.to_account_info(), accounts);
        burn(ctx, amount)
    }

}