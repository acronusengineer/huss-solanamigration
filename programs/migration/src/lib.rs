use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer as SplTransfer};
// use solana_program::pubkey;

declare_id!("9bq1F9kiBcrCEPNHoNmCs4sZoY4v33gAoFmc6yijtmjU");

#[program]
pub mod forwarder {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        // msg!("keys: {}, {}", &ctx.accounts.user.key, &ctx.accounts.authorized_address);
        require!(&ctx.accounts.user.key == &ctx.accounts.authorized_address.key, CustomError::Unauthorized);
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn flush_spl_tokens(ctx: Context<FlushTokens>, amount:u64) -> Result<()> {
        require!(&ctx.accounts.from.key == &ctx.accounts.authorized_address.key, CustomError::Unauthorized);
        let destination = &ctx.accounts.to_ata;
        let source = &ctx.accounts.from_ata;
        let token_program = &ctx.accounts.token_program;
        let authority = &ctx.accounts.from;

        //Transfer tokens from taker to owner of this contract
        let cpi_accounts = SplTransfer {
            from: source.to_account_info().clone(),
            to: destination.to_account_info().clone(),
            authority: authority.to_account_info().clone(),
        };

        let cpi_program = token_program.to_account_info();

        token::transfer(CpiContext::new(cpi_program, cpi_accounts), amount)?;
        Ok(())
    }
}

pub mod forwarderfactory;

#[account]  
pub struct Forwarder {  
    pub owner: Pubkey,  
}  

#[derive(Accounts)]  
pub struct Initialize<'info> { 
    #[account(mut)]
    pub user: Signer<'info>,  
    /// The account that is authorized to call the function  
    /// CHECK: This is not dangerous
    pub authorized_address: AccountInfo<'info>,  
    pub system_program: Program<'info, System>,  
}  

#[derive(Accounts)]  
pub struct FlushTokens<'info> { 
    #[account(mut)]  
    pub from: Signer<'info>, 
    #[account(mut)]  
    pub from_ata: Account<'info, TokenAccount>,  
    #[account(mut)]  
    pub to_ata: Account<'info, TokenAccount>,  
    pub token_program: Program<'info, Token>,  
    /// CHECK: This is not dangerous 
    pub authorized_address: AccountInfo<'info>,  
}  

#[error_code]  
pub enum CustomError {  
    #[msg("Unauthorized access.")]  
    Unauthorized,  
}  