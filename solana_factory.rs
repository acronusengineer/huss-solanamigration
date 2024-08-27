use solana_program::{
    account_info::{next_account_info, AccountInfo},
    borsh::try_from_slice_unchecked,
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
};
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Factory {
    pub owner: Pubkey,
    pub forwarder_count: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum FactoryInstruction {
    CreateForwarder { salt: u64 },
}

entrypoint!(process_instruction);
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = FactoryInstruction::try_from_slice(instruction_data)?;
    let accounts_iter = &mut accounts.iter();

    match instruction {
        FactoryInstruction::CreateForwarder { salt } => {
            let factory_account = next_account_info(accounts_iter)?;
            let owner_account = next_account_info(accounts_iter)?;
            let forwarder_account = next_account_info(accounts_iter)?;
            let system_program_account = next_account_info(accounts_iter)?;

            let mut factory_data = Factory::try_from_slice(&factory_account.data.borrow())?;
            if factory_data.owner != *owner_account.key {
                return Err(ProgramError::IncorrectProgramId);
            }

            let (forwarder_pda, bump_seed) = Pubkey::find_program_address(
                &[factory_data.owner.as_ref(), &salt.to_le_bytes()],
                program_id,
            );

            let forwarder_signer_seeds: &[&[u8]] = &[factory_data.owner.as_ref(), &[bump_seed]];

            invoke_signed(
                &system_instruction::create_account(
                    owner_account.key,
                    &forwarder_pda,
                    Rent::get()?.minimum_balance(forwarder_account.data_len()),
                    forwarder_account.data_len() as u64,
                    program_id,
                ),
                &[
                    owner_account.clone(),
                    forwarder_account.clone(),
                    system_program_account.clone(),
                ],
                &[forwarder_signer_seeds],
            )?;

            factory_data.forwarder_count += 1;
            factory_data.serialize(&mut *factory_account.data.borrow_mut())?;
            msg!("Forwarder created at: {}", forwarder_pda);
        }
    }
    Ok(())
}
