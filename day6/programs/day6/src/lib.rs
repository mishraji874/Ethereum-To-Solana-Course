use anchor_lang::prelude::*;

declare_id!("CeQJK4PX4hjZMrqj6NJyJCeDro61hUbGCA5AKKCfAk7q");

#[program]
pub mod day6 {
    use super::*;

    // Using HashMap for Mapping purpose
    use std::collections::HashMap;
    pub fn initialize(ctx: Context<Initialize>, key: String, value: String) -> Result<()> {
        let mut my_map = HashMap::new();
        my_map.insert(key.to_string(), value.to_string());
        msg!("My name is {}", my_map[&key]);
        Ok(())
    }
    
    // pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    //     // For loop
    //     for i in (0..10).step_by(2) {
    //         msg!("{}", i);
    //     }

    //     // Fixed array
    //     let my_array: [u32; 5] = [10, 20, 30, 40, 50];
    //     let first_element = my_array[0];
    //     let third_element = my_array[2];
    //     let mut mutable_array: [u32; 3] = [100, 200, 300];
    //     mutable_array[1] = 250;

    //     // Dynamic array
    //     let mut dynamic_array: Vec<u32> = Vec::new();
    //     dynamic_array.push(10);
    //     dynamic_array.push(20);
    //     dynamic_array.push(30);
    //     let first_element = dynamic_array[0];
    //     let third_element = dynamic_array[2];
    //     msg!("Third element = {}", third_element);
    //     Ok(())
    // }

    pub fn age_checker(ctx: Context<Initialize>, age: u64) -> Result<()> {
        if age >= 18 {
            msg!("You are 18 years old or above");
        } else {
            msg!("You are below 18 years old");
        }
        Ok(())
    }

    pub fn age_checker_ternary(ctx: Context<Initialize>, age: u64) -> Result<()> {
        let result = if age >= 18 {"You are 18 years old or above"} else { "You are below 18 years old" };
        msg!("{:?}", result);
        Ok(())
    }
    
    pub fn age_checker_match(ctx: Context<Initialize>, age: u64) -> Result<()> {
        match age {
            1 => {
                // Code block executed if age equals 1
                msg!("The age is 1");
            },
            2 | 3 => {
                // Code block executed if age equals 2 or 3
                msg!("The age is either 2 or 3");
            },
            4..=6 => {
                // Code block executed if age is in the 
                // range 4 to 6 (inclusive)
                msg!("The age is between 4 and 6");
            },
            _ => {
                // Code block executed for any other age
                msg!("The age is something else");
            }
        }
        Ok(())
    }
    
    
}

#[derive(Accounts)]
pub struct Initialize {}
