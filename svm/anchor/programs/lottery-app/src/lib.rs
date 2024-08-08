use anchor_lang::prelude::*;

declare_id!("C1HzcdTw9f5rEyrEHW6FnmdWo9HuxY4gwW9AJ18EgHJw");

#[program]
pub mod lottery_app {
    use super::*;


    pub fn verify_proof(ctx: Context<Initialize>, proof: Vec<u8>, public_inputs: Vec<[u8; 32]>) -> Result<()> {
        msg!("Proof verified!");
        println!("{:?}", proof);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
