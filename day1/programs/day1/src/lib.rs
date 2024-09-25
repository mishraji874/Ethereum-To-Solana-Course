use anchor_lang::prelude::*;

declare_id!("5WYZ61JaSv4daA49VKTiEYuvmBsLnHKsKJiy29d69kSv");

#[program]
pub mod day1 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Hello, world!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
