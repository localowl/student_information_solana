use anchor_lang::prelude::*;

#[account]
pub struct StudentAccount {
    pub name: String,
    pub biography: String,
    pub owner: Pubkey,
}

impl StudentAccount {
    pub const LEN: usize = 24 + 54 + 32;
}
