//! SOL program
#![allow(clippy::upper_case_acronyms)]
pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;
//https://paulx.dev/blog/2021/01/14/programming-on-solana-an-introduction/#trying-out-the-program-understanding-alice-s-transaction
#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint;

// Export current SDK types for downstream users building with a different SDK version
// pub use solana_program;

// solana_program::declare_id!("boxndjnzQZEWbBku3YipL4pchYRc1zi4nNSrFUwawWo");
