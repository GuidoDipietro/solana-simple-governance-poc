use anchor_lang::prelude::*;

#[account]
pub struct GlobalState {
    pub admin: Pubkey,
    pub value: u8,
}

impl GlobalState {
    pub const LEN: usize = 8 + 32 + 1;
}
