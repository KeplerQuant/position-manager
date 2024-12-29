use std::rc::Rc;

use anchor_client::Program;
use anchor_lang::system_program;
use anyhow::Result;
use solana_sdk::{instruction::Instruction, pubkey::Pubkey, signature::Keypair};

use crate::position::get_position_pda;

pub fn close_position_instruction(
    program: &Program<Rc<Keypair>>,
    position_mint: Pubkey,
) -> Result<Vec<Instruction>> {
    let nft_ata_token_account = spl_associated_token_account::get_associated_token_address(
        &program.payer(),
        &position_mint,
    );
    let personal_position_key = get_position_pda(&position_mint, &raydium_amm_v3::ID);

    let instructions = program
        .request()
        .accounts(position::accounts::AdjustPosition {
            clmm_program: raydium_amm_v3::ID,
            nft_owner: program.payer(),
            position_nft_mint: position_mint,
            position_nft_account: nft_ata_token_account,
            personal_position: personal_position_key,
            system_program: system_program::ID,
            token_program_2022: spl_token_2022::id(),
        })
        .args(position::instruction::AdjustPosition {})
        .instructions()?;

    Ok(instructions)
}
