use anchor_lang::prelude::*;

declare_id!("BLk2YTz9yMXuYsbSCDatkEYVetHxsxM1WwKmWJAgsXZB");

pub mod instructions;

use crate::instructions::*;

#[program]
pub mod governance {
    use super::*;

    pub fn edit_mock_state(ctx: Context<EditMockState>, value: u8) -> Result<()> {
        edit_mock_state::edit_mock_state_handler(ctx, value)
    }
}
