//! Event callbacks for Charta VM
//!
//! Provides callback system for reacting to VM events like coil state changes
//! and cycle completion.

use std::collections::HashMap;
use std::sync::Arc;

/// Callback function type for coil state changes
pub type CoilChangeCallback = Arc<dyn Fn(&str, bool, bool) + Send + Sync>;

/// Callback function type for cycle completion
pub type CycleCompleteCallback = Arc<dyn Fn(&HashMap<String, bool>) + Send + Sync>;

/// Event callback manager
pub struct CallbackManager {
    /// Callbacks for coil state changes: coil_name -> callback
    coil_callbacks: HashMap<String, Vec<CoilChangeCallback>>,
    /// Callback for cycle completion
    cycle_complete_callback: Option<CycleCompleteCallback>,
}

impl CallbackManager {
    /// Create a new callback manager
    pub fn new() -> Self {
        Self {
            coil_callbacks: HashMap::new(),
            cycle_complete_callback: None,
        }
    }

    /// Register a callback for a specific coil state change
    ///
    /// The callback receives: (coil_name, old_value, new_value)
    pub fn on_coil_change<F>(&mut self, coil_name: &str, callback: F)
    where
        F: Fn(&str, bool, bool) + Send + Sync + 'static,
    {
        self.coil_callbacks
            .entry(coil_name.to_string())
            .or_insert_with(Vec::new)
            .push(Arc::new(callback));
    }

    /// Register a callback for all coil changes
    pub fn on_any_coil_change<F>(&mut self, callback: F)
    where
        F: Fn(&str, bool, bool) + Send + Sync + 'static,
    {
        self.on_coil_change("*", callback);
    }

    /// Register a callback for cycle completion
    ///
    /// The callback receives the outputs map (coil_name -> new_state)
    pub fn on_cycle_complete<F>(&mut self, callback: F)
    where
        F: Fn(&HashMap<String, bool>) + Send + Sync + 'static,
    {
        self.cycle_complete_callback = Some(Arc::new(callback));
    }

    /// Trigger callbacks for coil changes
    pub fn trigger_coil_changes(&self, changes: &HashMap<String, (bool, bool)>) {
        for (coil_name, (old_value, new_value)) in changes {
            // Call specific callbacks for this coil
            if let Some(callbacks) = self.coil_callbacks.get(coil_name) {
                for callback in callbacks {
                    callback(coil_name, *old_value, *new_value);
                }
            }

            // Call wildcard callbacks
            if let Some(callbacks) = self.coil_callbacks.get("*") {
                for callback in callbacks {
                    callback(coil_name, *old_value, *new_value);
                }
            }
        }
    }

    /// Trigger cycle complete callback
    pub fn trigger_cycle_complete(&self, outputs: &HashMap<String, bool>) {
        if let Some(callback) = &self.cycle_complete_callback {
            callback(outputs);
        }
    }

    /// Clear all callbacks
    pub fn clear(&mut self) {
        self.coil_callbacks.clear();
        self.cycle_complete_callback = None;
    }

    /// Remove callbacks for a specific coil
    pub fn remove_coil_callbacks(&mut self, coil_name: &str) {
        self.coil_callbacks.remove(coil_name);
    }
}

impl Default for CallbackManager {
    fn default() -> Self {
        Self::new()
    }
}
