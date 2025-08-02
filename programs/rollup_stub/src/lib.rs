#![allow(unexpected_cfgs, deprecated)]
use anchor_lang::prelude::*;

declare_id!("8mWjeFrmowFrSKTAaWrstBcbXeMJUGpLTX71pAhKGsqP");

#[program]
pub mod rollup_stub {
    use super::*;

    pub fn submit_root(ctx: Context<SubmitRoot>, root: [u8; 32]) -> Result<()> {
        ctx.accounts.state_root.root = root;
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SubmitRoot<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init_if_needed,
        seeds = [b"rollup", signer.key().as_ref()],
        bump,
        payer = signer,
        space = 8 + 32
    )]
    pub state_root: Account<'info, StateRoot>,

    pub system_program: Program<'info, System>
}


#[account]
pub struct StateRoot{
    pub root: [u8; 32]
}