//! Coil management for Charta VM

use crate::error::{Error, Result};

/// Coil manager for reading coil states
pub struct CoilManager;

impl CoilManager {
    /// Create a new coil manager
    pub fn new() -> Self {
        Self
    }

    /// Validate coil name
    pub fn validate_name(name: &str) -> Result<()> {
        if name.is_empty() {
            return Err(Error::InvalidOperation("Coil name cannot be empty".to_string()));
        }
        Ok(())
    }
}

impl Default for CoilManager {
    fn default() -> Self {
        Self::new()
    }
}
