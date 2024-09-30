use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};

pub fn initialize_token_mint(_ctx: Context<InitializeMint>) -> Result<()> {
    msg!("Token mint initialized");
    Ok(())
}

#[derive(Accounts)]
pub struct InitializeMint<'info> {
    #[account(
        init,
        seeds= ["mint".as_bytes()],
        bump,
        payer = owner,
        mint::decimals = 6,
        mint::authority = owner,
    )]
    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub owner: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}
