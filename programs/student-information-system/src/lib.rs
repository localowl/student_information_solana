pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("8fKgM4qu2FztJwheT5mNou55bnDynkJzMzeEVKXdWNpA");

#[program]
pub mod student_information_system {

    use super::*;

    pub fn add_student(ctx: Context<AddStudent>, name: String, biography: String) -> Result<()> {
        add_student::add_student(ctx, name, biography)
    }

    pub fn update_student(
        ctx: Context<UpdateStudent>,
        name: String,
        biography: String,
    ) -> Result<()> {
        update_student::update_student(ctx, name, biography)
    }

    pub fn delete_student(ctx: Context<DeleteStudent>, name: String) -> Result<()> {
        delete_student::delete_student(ctx, name)
    }

    pub fn initialize_token_mint(ctx: Context<InitializeMint>) -> Result<()> {
        initialize_mint::initialize_token_mint(ctx)
    }
}
