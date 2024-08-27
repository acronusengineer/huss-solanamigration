use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    pubkey::Pubkey,
    system_instruction,
    sysvar::{rent::Rent, Sysvar},
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Forwarder {
    pub owner: Pubkey,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum ForwarderInstruction {
    InitForwarder { owner: Pubkey },
    TransferTokens { amount: u64, destination: Pubkey },
}

entrypoint!(process_instruction);
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = ForwarderInstruction::try_from_slice(instruction_data)?;
    let accounts_iter = &mut accounts.iter();

    match instruction {
        ForwarderInstruction::InitForwarder { owner } => {
            let forwarder_account = next_account_info(accounts_iter)?;
            let rent = Rent::get()?;
            if !rent.is_exempt(forwarder_account.lamports(), forwarder_account.data_len()) {
                return Err(ProgramError::AccountNotRentExempt);
            }
            let mut forwarder_data = Forwarder::try_from_slice(&forwarder_account.data.borrow())?;
            forwarder_data.owner = owner;
            forwarder_data.serialize(&mut *forwarder_account.data.borrow_mut())?;
            msg!("Forwarder initialized with owner: {}", owner);
        }
        ForwarderInstruction::TransferTokens { amount, destination } => {
            let forwarder_account = next_account_info(accounts_iter)?;
            let owner_account = next_account_info(accounts_iter)?;
            let token_program_account = next_account_info(accounts_iter)?;
            let destination_account = next_account_info(accounts_iter)?;

            let forwarder_data = Forwarder::try_from_slice(&forwarder_account.data.borrow())?;
            if forwarder_data.owner != *owner_account.key {
                return Err(ProgramError::IncorrectProgramId);
            }

            let transfer_instruction = spl_token::instruction::transfer(
                token_program_account.key,
                forwarder_account.key,
                destination_account.key,
                owner_account.key,
                &[],
                amount,
            )?;

            invoke(
                &transfer_instruction,
                &[
                    forwarder_account.clone(),
                    destination_account.clone(),
                    owner_account.clone(),
                ],
            )?;
            msg!("Transferred {} tokens to {}", amount, destination);
        }
    }
    Ok(())
}
