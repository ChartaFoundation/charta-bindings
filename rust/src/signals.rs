//! Signal management for Charta VM

use crate::error::{Error, Result};

/// Signal manager for setting and getting signal values
pub struct SignalManager;

impl SignalManager {
    /// Create a new signal manager
    pub fn new() -> Self {
        Self
    }

    /// Validate signal name
    pub fn validate_name(name: &str) -> Result<()> {
        if name.is_empty() {
            return Err(Error::InvalidOperation("Signal name cannot be empty".to_string()));
        }
        Ok(())
    }
}

impl Default for SignalManager {
    fn default() -> Self {
        Self::new()
    }
}
