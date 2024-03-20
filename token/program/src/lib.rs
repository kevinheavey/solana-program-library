#![allow(clippy::arithmetic_side_effects)]
#![deny(missing_docs)]
#![cfg_attr(not(test), forbid(unsafe_code))]

//! An ERC20-like Token program for the Solana blockchain

pub use spl_token_api::{
    amount_to_ui_amount, amount_to_ui_amount_string, amount_to_ui_amount_string_trimmed, check_id,
    check_program_account, error, id, instruction, native_mint, state, try_ui_amount_into_amount,
    ui_amount_to_amount, ID,
};
pub mod processor;

#[cfg(not(feature = "no-entrypoint"))]
mod entrypoint;

// Export current sdk types for downstream users building with a different sdk
// version
pub use solana_program;
