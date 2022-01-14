use anchor_lang::prelude::*;

declare_id!("4UQ8dKxHjwV8QpetmqF62Xqnf17DavL3EWf6b6DbfYmz");

#[program]
pub mod qwestive_stake {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
