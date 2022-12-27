use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod hello_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("init with zero");
        ctx.accounts.counter.value = 0;
        Ok(())
    }

    pub fn incr(ctx: Context<Incr>, delta: u32) -> Result<()> {
        msg!("old counter value is {}", ctx.accounts.counter.value);
        ctx.accounts.counter.value += delta;
        msg!("new counter value is {}", ctx.accounts.counter.value);
        Ok(())
    }
}

#[account]
pub struct Counter {
    pub value: u32,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub funding: Signer<'info>,
    #[account(init, payer = funding, space=8 + 4, seeds = [b"counter-seeds"], bump)]
    pub counter: Account<'info, Counter>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Incr<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
}
