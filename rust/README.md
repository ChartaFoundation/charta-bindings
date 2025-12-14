# Charta Rust SDK

Rust SDK for embedding the Charta VM in Rust applications.

## Features

- **Async/Await Support**: Built on Tokio for non-blocking execution
- **Type-Safe API**: Leverages Rust's type system for safety
- **VM Embedding**: Load and execute Charta programs
- **Signal/Coil Management**: Set signals and read coil states
- **Error Handling**: Comprehensive error types with `thiserror`

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
charta = { path = "../charta-bindings/rust" }
tokio = { version = "1.0", features = ["full"] }
```

## Usage

### Basic Example

```rust
use charta::{ChartaVM, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Create VM instance
    let mut vm = ChartaVM::new();

    // Load a program
    vm.load_program_from_file("program.ir.json").await?;

    // Set input signals
    vm.set_signal("user_submitted", true).await?;
    vm.set_signal("system_ok", true).await?;

    // Execute a scan cycle
    let outputs = vm.execute_cycle().await?;

    // Check coil states
    if outputs.get("allow_review").unwrap_or(&false) {
        println!("Review allowed!");
    }

    Ok(())
}
```

### With Input Signals

```rust
use charta::ChartaVM;
use std::collections::HashMap;

let mut vm = ChartaVM::new();
vm.load_program_from_file("program.ir.json").await?;

// Prepare inputs
let mut inputs = HashMap::new();
inputs.insert("submission_received".to_string(), true);
inputs.insert("governance_ok".to_string(), true);

// Execute with inputs
let outputs = vm.execute_cycle_with_inputs(inputs).await?;
```

### Reading States

```rust
// Get single coil state
let coil_state = vm.get_coil("allow_review").await?;

// Get all coils
let all_coils = vm.get_all_coils().await?;

// Get all signals
let all_signals = vm.get_all_signals().await?;
```

## API Reference

### ChartaVM

Main VM instance for executing Charta programs.

- `new()` - Create a new VM instance
- `load_program(ir_json)` - Load program from IR JSON string
- `load_program_from_file(path)` - Load program from file
- `execute_cycle()` - Execute one scan cycle
- `execute_cycle_with_inputs(inputs)` - Execute with input signals
- `set_signal(name, value)` - Set a signal value
- `get_signal(name)` - Get a signal state
- `get_coil(name)` - Get a coil state
- `get_all_signals()` - Get all signal states
- `get_all_coils()` - Get all coil states
- `signal_names()` - Get list of signal names
- `coil_names()` - Get list of coil names

## Error Handling

All operations return `Result<T, Error>` where `Error` is an enum covering:

- `VM` - VM execution errors
- `IRLoad` - IR loading/parsing errors
- `IO` - File I/O errors
- `JSON` - JSON parsing errors
- `NotFound` - Signal/coil not found
- `InvalidOperation` - Invalid operation attempted

## Status

**Phase 4: Rust SDK** - ✅ **Core Features Complete**

- [x] SDK structure and API design ✅
- [x] VM embedding wrapper ✅
- [x] Async/await support ✅
- [x] Signal/coil management ✅
- [x] Event callbacks ✅
- [x] Examples (4 examples) ✅
- [x] Integration tests (7 tests) ✅

## Event Callbacks

The SDK supports event callbacks for reacting to VM state changes:

### Coil Change Callbacks

Register callbacks for when specific coils change state:

```rust
vm.on_coil_change("allow_review", |name, old_val, new_val| {
    println!("Coil '{}' changed: {} → {}", name, old_val, new_val);
    if new_val {
        // Take action when coil energises
    }
}).await;
```

### Any Coil Change Callbacks

Register a callback for any coil change:

```rust
vm.on_any_coil_change(|name, old_val, new_val| {
    println!("Any coil changed: '{}' {} → {}", name, old_val, new_val);
}).await;
```

### Cycle Complete Callbacks

Register a callback for when each cycle completes:

```rust
vm.on_cycle_complete(|outputs| {
    println!("Cycle complete. Outputs: {:?}", outputs);
    // Process outputs
}).await;
```

## Examples

The SDK includes several examples:

- **basic.rs**: Simple program execution
- **callbacks.rs**: Event callback system demonstration
- **governance.rs**: Governance pattern implementation
- **multiple_cycles.rs**: State tracking across multiple cycles

Run examples with:

```bash
cargo run --example basic
cargo run --example callbacks
cargo run --example governance
cargo run --example multiple_cycles
```

## Development

```bash
# Check compilation
cargo check

# Run tests
cargo test

# Build
cargo build --release
```
