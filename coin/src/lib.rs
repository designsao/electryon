use solana_program::{
    account_info::AccountInfo,      // For handling Solana account information
    entrypoint,                     // Macro for declaring program entry point
    entrypoint::ProgramResult,      // Return type for Solana programs
    pubkey::Pubkey,                 // For handling Solana public keys
    program_error::ProgramError,    // For handling program errors
};
use borsh::{BorshDeserialize, BorshSerialize};

mod processor;
mod state;

use processor::Processor;

#[derive(BorshSerialize, BorshDeserialize)]
pub enum StablecoinInstruction {
    InitializeVault,                        // Create new vault
    DepositCollateral { amount: u64 },      // Deposit BTC collateral
    MintStablecoin { amount: u64 },         // Create new stablecoins
    RedeemStablecoin { amount: u64 },       // Redeem stablecoins for BTC
    WithdrawCollateral { amount: u64 },     // Withdraw BTC collateral
}

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,        // Program Address on Solana
    accounts: &[AccountInfo],   // Array of accounts that the instruction will interact with
    instruction_data: &[u8],    // Raw byte data of the instruction being called
) -> ProgramResult {
    // Deserialize the incoming instruction data into your StablecoinInstruction enum
    let instruction = StablecoinInstruction::try_from_slice(instruction_data)?;
    
    match instruction {
        StablecoinInstruction::InitializeVault => {
            Processor::process_initialize_vault(accounts, program_id)
        },
        StablecoinInstruction::DepositCollateral { amount } => {
            Processor::process_deposit_collateral(accounts, amount, program_id)
        },
        // ... other instruction handlers
    }
}