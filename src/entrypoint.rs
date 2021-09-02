use solana_program::{
    msg,
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, pubkey::Pubkey,
};
use std::convert::TryFrom;

// use crate::processor::Processor;

entrypoint!(process_instruction);
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("process_instruction");
    let received_data = String::from_utf8(instruction_data.to_vec()).unwrap();
    msg!("received_data = {:?}", received_data);
    // ProgramResult::default()
    Ok(())
    // Processor::process(program_id, accounts, instruction_data)
}