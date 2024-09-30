use crate::constants::*;
use crate::error::*;
use crate::state::*;
use anchor_lang::prelude::*;

pub fn update_student(
    ctx: Context<UpdateStudent>,
    new_name: String,
    new_biography: String,
) -> Result<()> {
    require!(
        new_name.len() <= MAX_NAME_LENGTH,
        StudentError::NameLengthError
    );

    require!(
        new_biography.len() <= MAX_BIOGRAPHY_LENGTH,
        StudentError::BiographyLengthError
    );

    msg!("Student Name: {}", new_name);
    msg!("Student Biography: {}", new_biography);

    let student = &mut ctx.accounts.student;
    student.name = new_name;
    student.biography = new_biography;

    Ok(())
}

#[derive(Accounts)]
#[instruction(name: String)]
pub struct UpdateStudent<'info> {
    #[account(
        mut,
        seeds = [name.as_bytes(), owner.key().as_ref()],
        bump,
        realloc = DISCRIMINATOR + StudentAccount::LEN,
        realloc::payer = owner,
        realloc::zero = true
    )]
    pub student: Account<'info, StudentAccount>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
