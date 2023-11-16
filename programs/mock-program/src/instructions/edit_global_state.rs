use anchor_lang::prelude::*;

use std::str::FromStr;
use crate::state::GlobalState;

#[derive(Accounts)]
pub struct EditGlobalState<'info> {
    #[account(
        mut,
        constraint =
            admin.key == &global_state.admin ||
            admin.key == &governance_pda.key()
    )]
    pub admin: Signer<'info>,

    /// CHECK: Used to derive address
    #[account(
        seeds = [b"governance_pda"],
        bump,
        seeds::program = Pubkey::from_str("BLk2YTz9yMXuYsbSCDatkEYVetHxsxM1WwKmWJAgsXZB").unwrap()
    )]
    pub governance_pda: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [b"global_state"],
        bump,
    )]
    pub global_state: Account<'info, GlobalState>,
}

pub fn edit_global_state_handler(ctx: Context<EditGlobalState>, value: u8) -> Result<()> {
    ctx.accounts.global_state.value = value;

    Ok(())
}
