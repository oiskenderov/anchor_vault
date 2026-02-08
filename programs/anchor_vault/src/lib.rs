#![allow(unexpected_cfgs)]
#![allow(deprecated)]
// #![allow(unused)]

pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;
#[account]
#[derive(InitSpace)]
pub struct VaultState {
    pub auth_bump: u8,
    pub vault_bump: u8,
    pub state_bump: u8,    
}
// #[program]
// imp VaultState {
//     pub const LEN: usize = 8 + 1 + 1 + 1;
// }

pub use constants::*;
pub use instructions::*;

declare_id!("DmE9FvxBR1SX3hRtxAcrUG7kvawmwtoTE4f4Ze46P4o6");

#[program]
pub mod anchor_vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.init(ctx.bumps)  
    }

     pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        ctx.accounts.withdraw(amount)
    }

    pub fn close(ctx: Context<Close>) -> Result<()> {
        ctx.accounts.close()
    }
}
