use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer as SplTransfer};
use solana_program::pubkey;

declare_id!("8ZYUprHFV6SJTtvpknRqgxy5BafB5kbosQxq3tbyAqZD");

#[program]
pub mod forwarder {
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

pub mod forwarder_factory {

    use super::*;
    
    pub fn create_forwarder(ctx: Context<CreateForwarder>) -> Result<()> {
        let forwarder_address = ctx.accounts.forwarder.key();  
        let forwarder = &mut ctx.accounts.forwarder;
        forwarder.owner = *ctx.accounts.authorized_address.key; 
        msg!("New forwarder created with address: {:?}", forwarder_address);
        msg!("Forwarder owner: {:?}", forwarder.owner);
        Ok(())
    }

    pub fn transfer_ownership(ctx: Context<ChangeAccountOwner>, new_owner:Pubkey) -> Result<()> {
        // require!(&ctx.accounts.forwarder.key() == ctx.accounts.current_owner.key, CustomError::Unauthorized);
        let pastowner = &mut ctx.accounts.forwarder.key();
        //Update the owner of the forwarder to the new owner
        let forwarder = &mut ctx.accounts.forwarder;   
        forwarder.owner = new_owner;

          // Log the ownership transfer  
          msg!("Ownership of forwarder at {:?} transferred to {:?}", pastowner, forwarder.owner); 
        Ok(())
    }

    pub fn flush_tokens_from_list(ctx: Context<FlushTokenFromList>, amount:u64, accountlist: Vec<Pubkey>) -> Result<()> {
        for _account in accountlist {
                 // Ensure the account is authorized to flush tokens  
                require!(&ctx.accounts.from.key == &ctx.accounts.authorized_address.key, CustomError::Unauthorized); 
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
}  


#[derive(Accounts)]  
pub struct Initialize<'info> {  
    #[account(init, payer = user, space = 8 + std::mem::size_of::<Forwarder>())]  
    pub forwarder: Account<'info, Forwarder>,  
    #[account(mut, address=pubkey!("11111111111111111111111111111111"))]
    pub user: Signer<'info>,  
    /// The account that is authorized to call the function  
    /// CHECK: This is not dangerous
    pub authorized_address: AccountInfo<'info>,  
    pub system_program: Program<'info, System>,  
}  

#[derive(Accounts)]  
pub struct FlushTokens<'info> { 
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
    #[account(mut)]
    pub forwarder: Account<'info, Forwarder>,
    #[account(mut)]
    pub current_owner: Signer<'info>,
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