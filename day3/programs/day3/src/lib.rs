use anchor_lang::prelude::*;

declare_id!("6DARUzBi7bwNS5sEFK24Y328nCxTv4TTaYV1ckgd3Mtz");

#[program]
pub mod day3 {
    use super::*;

    pub fn boaty_mc_boatface(ctx: Context<BoatyMcBoatface>) -> Result<()> {
        Ok(())
    }

    pub fn add(ctx: Context<BoatyMcBoatface>, a: u64, b: u64) -> Result<()> {
        let sum = a + b;
        msg!("Sum is {}", sum);  
        Ok(())
    }
    
    pub fn sub(ctx: Context<BoatyMcBoatface>, a: u64, b: u64) -> Result<()> {
        let difference = a - b;
        msg!("Difference is {}", difference);  
        Ok(())
    }
    
}

#[derive(Accounts)]
pub struct BoatyMcBoatface {}
