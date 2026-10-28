use anchor_lang::prelude::*;

declare_id!("U5rMvJTxeTbiGVzr5dX5f2jqWBXUmeeWRrgjbHGHiGM");

#[program]
pub mod lockstep {
    use super::*;

    // 1. Create the Escrow
    pub fn initialize_escrow(
        ctx: Context<InitializeEscrow>, 
        amount: u64, 
        freelancer: Pubkey
    ) -> Result<()> {
        let escrow = &mut ctx.accounts.escrow;
        
        escrow.client = ctx.accounts.client.key();
        escrow.freelancer = freelancer;
        escrow.amount = amount;
        escrow.is_funded = false;
        escrow.bump = ctx.bumps.escrow;

        msg!("Escrow created! Amount: {}", escrow.amount);
        Ok(())
    }

    // 2. Deposit Funds into the Escrow
    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        let escrow = &mut ctx.accounts.escrow;

        // Security checks
        require!(!escrow.is_funded, EscrowError::AlreadyFunded);
        require!(amount == escrow.amount, EscrowError::InvalidAmount);

        // Transfer SOL using native Solana instructions
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.client.key(),
            &escrow.key(),
            amount,
        );
        
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.client.to_account_info(),
                escrow.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;

        // Mark as funded
        escrow.is_funded = true;

        msg!("Escrow funded with {} lamports!", amount);
        Ok(())
    }
}

// --- DATA STRUCTURES ---

#[account]
pub struct Escrow {
    pub client: Pubkey,
    pub freelancer: Pubkey,
    pub amount: u64,
    pub is_funded: bool,
    pub bump: u8,
}

// --- ACCOUNT VALIDATION ---

#[derive(Accounts)]
pub struct InitializeEscrow<'info> {
    #[account(mut)]
    pub client: Signer<'info>,

    #[account(
        init,
        payer = client,
        space = 8 + 32 + 32 + 8 + 1 + 1,
        seeds = [b"escrow", client.key().as_ref()],
        bump
    )]
    pub escrow: Account<'info, Escrow>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub client: Signer<'info>,

    #[account(
        mut,
        seeds = [b"escrow", client.key().as_ref()],
        bump = escrow.bump,
        constraint = escrow.client == client.key()
    )]
    pub escrow: Account<'info, Escrow>,

    pub system_program: Program<'info, System>,
}

// --- ERRORS ---

#[error_code]
pub enum EscrowError {
    #[msg("This escrow has already been funded.")]
    AlreadyFunded,
    #[msg("The deposit amount does not match the escrow amount.")]
    InvalidAmount,
}