use anchor_lang::prelude::*;

declare_id!("ALKKgt9LJuf7RzyNh2B6dVjwBh7eWC8DAu1T3dRvoic7");

#[program]
pub mod day5 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
