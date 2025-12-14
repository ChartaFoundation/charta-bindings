//! Charta Rust SDK
//!
//! Embed the Charta VM in Rust applications with a type-safe, async-friendly API.
//!
//! # Example
//!
//! ```no_run
//! use charta::{ChartaVM, Error};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Error> {
//!     // Create VM instance
//!     let mut vm = ChartaVM::new();
//!
//!     // Load a program
//!     vm.load_program_from_file("program.ir.json").await?;
//!
//!     // Set input signals
//!     vm.set_signal("user_submitted", true).await?;
//!     vm.set_signal("system_ok", true).await?;
//!
//!     // Execute a scan cycle
//!     let outputs = vm.execute_cycle().await?;
//!
//!     // Check coil states
//!     if *outputs.get("allow_review").unwrap_or(&false) {
//!         println!("Review allowed!");
//!     }
//!
//!     Ok(())
//! }
//! ```

pub mod vm;
pub mod execution;
pub mod signals;
pub mod coils;
pub mod callbacks;
pub mod error;

pub use vm::ChartaVM;
pub use error::{Error, Result};
pub use callbacks::{CallbackManager, CoilChangeCallback, CycleCompleteCallback};
