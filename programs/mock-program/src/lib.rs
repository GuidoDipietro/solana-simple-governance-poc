use anchor_lang::prelude::*;

declare_id!("8aSet5fdFgc3HvpVH6uX9FTWALEoGDyvs3rmqWaKPsDR");

pub mod instructions;
pub mod state;

pub use crate::{instructions::*, state::*};

#[program]
pub mod mock_program {
    use super::*;

    pub fn init_global_state(ctx: Context<InitGlobalState>, value: u8) -> Result<()> {
        init_global_state::init_global_state_handler(ctx, value)
    }

    pub fn edit_global_state(ctx: Context<EditGlobalState>, value: u8) -> Result<()> {
        edit_global_state::edit_global_state_handler(ctx, value)
    }
}
