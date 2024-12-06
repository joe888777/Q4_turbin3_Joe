use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{ MasterEditionAccount, Metadata, MetadataAccount },
    token_interface::{Mint, TokenAccount, TokenInterface, transfer_checked, TransferChecked}
};

use crate::state::{Marketplace, Listing};
use crate::error::ErrorCode::NameTooLong;

#[derive(Accounts)]
pub struct  List <'info> {
    #[account(mut)]
    maker: Signer<'info>,
    #[account(
        seeds = [b"marketplace", marketplace.name.as_bytes()],
        bump = marketplace.bump
    )]
    pub marketplace: Account<'info, Marketplace>,
    pub maker_mint: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = maker_mint,
        associated_token::authority = maker
    )]
    pub maker_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init,
        payer = maker,
        associated_token::mint = maker_mint,
        associated_token::authority = listing
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    pub listing: Account<'info, Listing>,
    pub collection_mint: InterfaceAccount<'info, Mint>

}

// pub struct List <'info> {
//     #[account(
//         mut
//     )]
//     pub maker: Signer<'info>,
//     #[account(
//         seeds=[b"marketplace", marketplace.name.as_bytes()],
//         bump = marketplace.bump
//     )]
//     pub marketplace: Account<'info, Marketplace>,
//     pub maker_mint: InferfaceAccount<'info, Mint>,
//     #[account(
//         mut,
//         associated_token::mint = maker_mint,
//         associated_token::authority = maker,
//     )]
//     pub maker_ata: InferfaceAccount<'info, TokenAccount>,
//     #[account(
//         init,
//         payer = maker,
//         associated_token::mint = maker_mint,
//         associated_token::authority = maker,
//     )]
//     pub vault: InterfaceAccount<'info, TokenAccount>,
//     #[account(
//         init,
//         payer = maker,
//         seeds = [marketplace.key().as_ref(), maker_mint.key().as_ref()],
//         bump,
//         space = Listing::INIT_SPACE
//     )]
//     pub listing: Account<'info, Listing>,
//     pub collection_mint: InterfaceAccount<'info, Mint>,
//     #[account(
//         seeds = [
//             b"metadata",
//             metadata_program.key().as_ref(),
//             maker_mint.key().as_ref(),
//             b"edition"
//         ],
//         seeds::program = metadata_program.key(),
//         bump,
//         constraint = metadata.collection.as_ref().unwrap().key.as_ref() == collection_mint.key().as_ref(),
//         constraint = metadata.collection.as_ref().unwrap().verified == true,
//     )]
//     pub metadata: Account<'info, MetadataAccount>,
//     pub master_edition: Account<'info, MasterEditionAccount>,
//     pub metadata_program: Program<'info, Metadata>,
//     pub associated_token_program: Program<'info, AssociatedToken>,
//     pub system_program: Program<'info, System>,
//     pub token_program: Program<'info, TokenInterface>
// }

impl <'info> List <'info> {
    pub fn create_listing (&mut self, price: u64, bumps: &ListingBumps) -> Result<()> {
        self.listing.set_inner(Listing {
            maker: self.maker.key(),
            mint: self.maker_mint.key(),
            price,
            bump: bumps.listing
        });
        Ok(())
    }
    pub fn deposit_nft (&mut self) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = TransferChecked {
            from: self.maker_ata.to_account_info(),
            to: self.vault.to_account_info(),
            mint: self.maker_mint.to_account_info(),
            authority: self.maker.to_account_info()
        };
        let cpi_ctx = CpiContext::new (cpi_program, cpi_accounts);
        transfer_checked(cpi_ctx, 1, self.maker_mint.decimal)?;
        Ok(())
    }
}