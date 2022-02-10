use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let source_info = next_account_info(account_info_iter)?;
    let dest_info = next_account_info(account_info_iter)?;

    **source_info.try_borrow_mut_lamports()? -= 5;
    **dest_info.try_borrow_mut_lamports()? += 5;
    Ok(())
}
