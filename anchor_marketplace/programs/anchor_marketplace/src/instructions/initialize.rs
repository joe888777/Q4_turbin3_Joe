use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface};

use crate::error::ErrorCode;
use crate::state::Marketplace;

#[derive(Accounts)]
#[instruction(name: string)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        payer = admin,
        seeds = [b"marketplace", name.as_bytes()],
        space = Marketplace::INIT_SPACE,
        bump
    )]
    pub marketplace: Account<'info, Marketplace>,

    #[account(
        seeds = [b"treasury", marketplace.key().as_ref()],
        bump
    )]
    pub treasury: SystemAccount<'info>,

    #[account(
        init,
        payer = admin,
        seeds = [b"rewards", marketplace.key().as_ref()],
        bump,
        mint::decimal = 6,
        mint::authority = marketplace
    )]
    pub reward_mint: InterfaceAccount<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, TokenInterface>,
}

impl<'info> Initialize<'info> {
    pub fn init(
        &mut self,
        name: String,
        fee: u16,
        bumps: &InitializeBumps
    ) -> Result<()> {
        require!(
            !name.is_empty() && name.len() < 4 + 33,
            ErrorCode::NameTooLong
        );

        self.marketplace.set_inner(Marketplace {
            admin: self.admin.key(),
            fee: fee,
            bump: bumps.marketplace,
            treasury_bump: bumps.treasury,
            reward_bump: bumps.reward,
            name,
        });
        Ok(())
    }
}
