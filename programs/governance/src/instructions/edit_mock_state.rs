use anchor_lang::prelude::*;
use mock_program::program::MockProgram;

#[derive(Accounts)]
pub struct EditMockState<'info> {
    /// CHECK: Used to derive PDA
    #[account(
        mut,
        seeds = [b"governance_pda"],
        bump,
    )]
    pub governance_pda: UncheckedAccount<'info>,

    /// CHECK: Checked in CPI
    #[account(
        mut,
        seeds = [b"global_state"],
        bump,
        seeds::program = mock_program.key(),
    )]
    pub mock_program_global_state: UncheckedAccount<'info>,

    pub mock_program: Program<'info, MockProgram>,
}

pub fn edit_mock_state_handler(ctx: Context<EditMockState>, value: u8) -> Result<()> {
    let signer_seeds = &[b"governance_pda".as_ref(), &[ctx.bumps.governance_pda]];

    mock_program::cpi::edit_global_state(
        CpiContext::new_with_signer(
            ctx.accounts.mock_program.to_account_info(),
            mock_program::cpi::accounts::EditGlobalState {
                admin: ctx.accounts.governance_pda.to_account_info(),
                governance_pda: ctx.accounts.governance_pda.to_account_info(),
                global_state: ctx.accounts.mock_program_global_state.to_account_info(),
            },
            &[signer_seeds],
        ),
        value,
    )?;

    Ok(())
}
