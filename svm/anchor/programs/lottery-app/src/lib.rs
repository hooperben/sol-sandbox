use anchor_lang::prelude::*;

declare_id!("C1HzcdTw9f5rEyrEHW6FnmdWo9HuxY4gwW9AJ18EgHJw");

#[program]
pub mod lottery_app {
    use super::*;

    pub fn greet(_ctx: Context<Initialize>) -> Result<()> {
        msg!("GM!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
