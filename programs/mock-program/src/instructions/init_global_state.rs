use anchor_lang::prelude::*;

use crate::state::GlobalState;

#[derive(Accounts)]
pub struct InitGlobalState<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        seeds = [b"global_state"],
        bump,
        space = GlobalState::LEN,
        payer = admin,
    )]
    pub global_state: Account<'info, GlobalState>,

    pub system_program: Program<'info, System>,
}

pub fn init_global_state_handler(ctx: Context<InitGlobalState>, value: u8) -> Result<()> {
    ctx.accounts.global_state.admin = ctx.accounts.admin.key();
    ctx.accounts.global_state.value = value;

    Ok(())
}
