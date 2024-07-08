#![allow(clippy::arithmetic_side_effects)]
pub mod entrypoint;
pub mod error;
pub mod instruction;
pub mod processor;
pub mod spl_utils;
pub mod state;
pub mod system_utils;
pub mod validation_utils;
// Export current sdk types for downstream users building with a different sdk
// version
pub use {solana_program, spl_program_ids::spl_binary_option::*};
