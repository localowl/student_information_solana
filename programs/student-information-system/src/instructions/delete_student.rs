use crate::state::*;
use anchor_lang::prelude::*;

pub fn delete_student(_ctx: Context<DeleteStudent>, name: String) -> Result<()> {
    msg!("Deleted Student: {}", name);

    Ok(())
}

#[derive(Accounts)]
#[instruction(name: String)]
pub struct DeleteStudent<'info> {
    #[account(
        mut,
        seeds = [name.as_bytes(), owner.key().as_ref()],
        bump,
        close = owner
    )]
    pub student: Account<'info, StudentAccount>,

    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
