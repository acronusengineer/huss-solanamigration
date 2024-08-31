use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Transfer as SplTransfer};
// use solana_program::pubkey;

declare_id!("9bq1F9kiBcrCEPNHoNmCs4sZoY4v33gAoFmc6yijtmjU");

#[program]
pub mod forwarder {
    use anchor_spl::token;

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
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

        //Transfer tokens contract to other
        let cpi_accounts = SplTransfer {
            from: source.to_account_info().clone(),
            to: destination.to_account_info().clone(),
            authority: authority.to_account_info().clone(),
        };

        let cpi_program = token_program.to_account_info();

        token::transfer(CpiContext::new(cpi_program, cpi_accounts), amount)?;
        Ok(())
    }

    pub fn create_forwarder(ctx: Context<CreateForwarder>, owner:Pubkey) -> Result<()> {
        let forwarder = &mut ctx.accounts.forwarder;
        forwarder.owner = owner; 
        msg!("New forwarder created and Forwarder owner: {:?}", forwarder.owner);
        Ok(())
    }

    pub fn transfer_ownership(ctx: Context<ChangeAccountOwner>, new_owner:Pubkey) -> Result<()> {
          // Change the contract owner to new owner
          ctx.accounts.forwarder.owner = new_owner;
          // Log the ownership transfer  
          msg!("Ownership of forwarder transferred to {:?}", new_owner); 
        Ok(())
    }

    pub fn flush_tokens_from_list(ctx: Context<FlushTokenFromList>, amount:u64, accountlist: Vec<Pubkey>) -> Result<()> {
        for _account in accountlist {
                 // Ensure the account is authorized to flush tokens  
                require!(&ctx.accounts.from.key == &ctx.accounts.authorized_address.owner, CustomError::Unauthorized); 
                 let source = &ctx.accounts.from_ata;
                 // Call the existing flush_spl_tokens function
                 let destination = &ctx.accounts.to_ata;
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
        }
        Ok(())
    }
}

#[account]  
pub struct Forwarder {  
    pub owner: Pubkey,  
    bump: u8
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

#[derive(Accounts)]  
pub struct CreateForwarder<'info> {  
    #[account(  
        init,  
        seeds = [b"forwarder", authorized_address.key().as_ref()],  
        bump,  
        payer = user,  
        space = 8 + std::mem::size_of::<Forwarder>()  
    )]  
    pub forwarder: Account<'info, Forwarder>,  
    #[account(mut)]  
    pub user: Signer<'info>,
    /// CHECK: This is not dangerous 
    pub authorized_address: AccountInfo<'info>,  
    pub system_program: Program<'info, System>,  
}  

#[derive(Accounts)]
pub struct ChangeAccountOwner<'info> {  
    /// CHECK: This is not dangerous
    pub authorized_address: AccountInfo<'info>,  
    #[account(  
        mut,  
        seeds = [b"forwarder", authorized_address.key().as_ref()],  
        bump = forwarder.bump,  
    )]  
    pub forwarder: Account<'info, Forwarder>,  
}  

#[derive(Accounts)]
pub struct FlushTokenFromList<'info> {
    pub from: Signer<'info>,
    #[account(mut)]
    pub from_ata: Account<'info, TokenAccount>,
    #[account(mut)]
    pub to_ata: Account<'info, TokenAccount>,
    pub token_program: Program<'info,Token>,
    /// CHECK: This is not dangerous
    pub authorized_address: AccountInfo<'info>, 
    // pub bumps: ForwarderBumps,   
}


#[error_code]  
pub enum CustomError {  
    #[msg("Unauthorized access.")]  
    Unauthorized,  
}  

