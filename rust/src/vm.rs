//! Charta VM wrapper for Rust SDK

use crate::error::{Error, Result};
use crate::callbacks::CallbackManager;
use charta_vm::{VM, ir::load_ir};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Charta VM instance for embedding in Rust applications
///
/// This is the main entry point for using Charta in Rust. It provides
/// an async-friendly API for loading programs, setting signals, executing
/// cycles, and reading coil states.
pub struct ChartaVM {
    /// Internal VM instance (wrapped in Arc for async sharing)
    vm: Arc<RwLock<VM>>,
    /// Callback manager for event handling
    callbacks: Arc<RwLock<CallbackManager>>,
}

impl ChartaVM {
    /// Create a new Charta VM instance
    pub fn new() -> Self {
        Self {
            vm: Arc::new(RwLock::new(VM::new())),
            callbacks: Arc::new(RwLock::new(CallbackManager::new())),
        }
    }

    /// Load a program from IR JSON string
    pub async fn load_program(&mut self, ir_json: &str) -> Result<()> {
        let ir = load_ir(ir_json)
            .map_err(|e| Error::IRLoad(e.to_string()))?;
        
        let mut vm = self.vm.write().await;
        vm.load_program(ir)
            .map_err(Error::VM)?;
        
        Ok(())
    }

    /// Load a program from a file
    pub async fn load_program_from_file<P: AsRef<std::path::Path>>(
        &mut self,
        path: P,
    ) -> Result<()> {
        let contents = tokio::fs::read_to_string(path).await?;
        self.load_program(&contents).await
    }

    /// Execute one scan cycle
    ///
    /// Returns a map of coil names to their new states (true if energised).
    /// Triggers callbacks for coil changes and cycle completion.
    pub async fn execute_cycle(&mut self) -> Result<HashMap<String, bool>> {
        // Get old coil states before execution
        let old_coils = {
            let vm = self.vm.read().await;
            vm.get_all_coils()
        };

        // Execute cycle
        let outputs = {
            let mut vm = self.vm.write().await;
            let inputs = HashMap::new();
            vm.step(inputs).map_err(Error::VM)?
        };

        // Calculate changes and trigger callbacks
        let changes: HashMap<String, (bool, bool)> = outputs
            .iter()
            .filter_map(|(name, &new_value)| {
                let old_value = old_coils.get(name).copied().unwrap_or(false);
                if old_value != new_value {
                    Some((name.clone(), (old_value, new_value)))
                } else {
                    None
                }
            })
            .collect();

        // Trigger callbacks
        if !changes.is_empty() {
            let callbacks = self.callbacks.read().await;
            callbacks.trigger_coil_changes(&changes);
        }

        let callbacks = self.callbacks.read().await;
        callbacks.trigger_cycle_complete(&outputs);

        Ok(outputs)
    }

    /// Execute one scan cycle with input signals
    ///
    /// Triggers callbacks for coil changes and cycle completion.
    pub async fn execute_cycle_with_inputs(
        &mut self,
        inputs: HashMap<String, bool>,
    ) -> Result<HashMap<String, bool>> {
        // Get old coil states before execution
        let old_coils = {
            let vm = self.vm.read().await;
            vm.get_all_coils()
        };

        // Execute cycle
        let outputs = {
            let mut vm = self.vm.write().await;
            vm.step(inputs).map_err(Error::VM)?
        };

        // Calculate changes and trigger callbacks
        let changes: HashMap<String, (bool, bool)> = outputs
            .iter()
            .filter_map(|(name, &new_value)| {
                let old_value = old_coils.get(name).copied().unwrap_or(false);
                if old_value != new_value {
                    Some((name.clone(), (old_value, new_value)))
                } else {
                    None
                }
            })
            .collect();

        // Trigger callbacks
        if !changes.is_empty() {
            let callbacks = self.callbacks.read().await;
            callbacks.trigger_coil_changes(&changes);
        }

        let callbacks = self.callbacks.read().await;
        callbacks.trigger_cycle_complete(&outputs);

        Ok(outputs)
    }

    /// Get the current state of a coil
    pub async fn get_coil(&self, name: &str) -> Result<Option<bool>> {
        let vm = self.vm.read().await;
        Ok(vm.get_coil_state(name))
    }

    /// Get the current state of a signal
    pub async fn get_signal(&self, name: &str) -> Result<Option<bool>> {
        let vm = self.vm.read().await;
        Ok(vm.get_signal_state(name))
    }

    /// Get all coil states
    pub async fn get_all_coils(&self) -> Result<HashMap<String, bool>> {
        let vm = self.vm.read().await;
        Ok(vm.get_all_coils())
    }

    /// Get all signal states
    pub async fn get_all_signals(&self) -> Result<HashMap<String, bool>> {
        let vm = self.vm.read().await;
        Ok(vm.get_all_signals())
    }

    /// Set a signal value
    pub async fn set_signal(&mut self, name: &str, value: bool) -> Result<()> {
        let mut vm = self.vm.write().await;
        vm.set_signal(name.to_string(), value);
        Ok(())
    }

    /// Set a coil value (for testing/debugging)
    pub async fn set_coil(&mut self, name: &str, value: bool) -> Result<()> {
        let mut vm = self.vm.write().await;
        vm.set_coil(name.to_string(), value);
        Ok(())
    }

    /// Get signal names
    pub async fn signal_names(&self) -> Result<Vec<String>> {
        let vm = self.vm.read().await;
        Ok(vm.signal_names().to_vec())
    }

    /// Get coil names
    pub async fn coil_names(&self) -> Result<Vec<String>> {
        let vm = self.vm.read().await;
        Ok(vm.coil_names().to_vec())
    }

    /// Register a callback for when a specific coil changes state
    ///
    /// The callback receives: (coil_name, old_value, new_value)
    pub async fn on_coil_change<F>(&self, coil_name: &str, callback: F)
    where
        F: Fn(&str, bool, bool) + Send + Sync + 'static,
    {
        let mut callbacks = self.callbacks.write().await;
        callbacks.on_coil_change(coil_name, callback);
    }

    /// Register a callback for when any coil changes state
    ///
    /// The callback receives: (coil_name, old_value, new_value)
    pub async fn on_any_coil_change<F>(&self, callback: F)
    where
        F: Fn(&str, bool, bool) + Send + Sync + 'static,
    {
        let mut callbacks = self.callbacks.write().await;
        callbacks.on_any_coil_change(callback);
    }

    /// Register a callback for cycle completion
    ///
    /// The callback receives the outputs map (coil_name -> new_state)
    pub async fn on_cycle_complete<F>(&self, callback: F)
    where
        F: Fn(&HashMap<String, bool>) + Send + Sync + 'static,
    {
        let mut callbacks = self.callbacks.write().await;
        callbacks.on_cycle_complete(callback);
    }

    /// Clear all callbacks
    pub async fn clear_callbacks(&self) {
        let mut callbacks = self.callbacks.write().await;
        callbacks.clear();
    }
}

impl Default for ChartaVM {
    fn default() -> Self {
        Self::new()
    }
}
