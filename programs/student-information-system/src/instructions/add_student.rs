use crate::constants::*;
use crate::error::*;
use crate::student::*;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{mint_to, Mint, MintTo, Token, TokenAccount};

pub fn add_student(ctx: Context<AddStudent>, name: String, biography: String) -> Result<()> {
    require!(name.len() <= MAX_NAME_LENGTH, StudentError::NameLengthError);

    require!(
        biography.len() <= MAX_BIOGRAPHY_LENGTH,
        StudentError::BiographyLengthError
    );

    msg!("Student Name: {}", name);
    msg!("Student Biography: {}", biography);

    let student = &mut ctx.accounts.student;
    student.owner = ctx.accounts.owner.key();
    student.name = name;
    student.biography = biography;

    mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                authority: ctx.accounts.owner.to_account_info(),
                to: ctx.accounts.token_account.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
            },
            &[&["mint".as_bytes(), &[ctx.bumps.mint]]],
        ),
        10 * 10 ^ 6,
    )?;

    msg!("Minted tokens");

    Ok(())
}
#[derive(Accounts)]
#[instruction(name: String)]
pub struct AddStudent<'info> {
    #[account(
        init,
        space = DISCRIMINATOR + StudentAccount::LEN,
        payer = owner,
        seeds = [name.as_bytes(), owner.key().as_ref()],
        bump
    )]
    pub student: Account<'info, StudentAccount>,

    #[account(
        mut,
        seeds = ["mint".as_bytes()],
        bump
    )]
    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,

    #[account(
        init_if_needed,
        payer = owner,
        associated_token::mint = mint,
        associated_token::authority = owner

    )]
    pub token_account: Account<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}
