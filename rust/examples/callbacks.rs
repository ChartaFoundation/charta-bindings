/// Example demonstrating event callbacks in the Charta Rust SDK

use charta::{ChartaVM, Error};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("Charta Rust SDK - Callbacks Example");

    // Create VM instance
    let mut vm = ChartaVM::new();

    // Create a simple IR program
    let ir_json = r#"
    {
        "version": "0.1.0",
        "module": {
            "name": "callback_example",
            "signals": [
                {"name": "input_signal"},
                {"name": "system_ok"}
            ],
            "coils": [
                {"name": "output_coil"},
                {"name": "critical_output"}
            ],
            "rungs": [
                {
                    "name": "test_rung",
                    "guard": {
                        "type": "contact",
                        "name": "input_signal",
                        "contact_type": "NO"
                    },
                    "actions": [
                        {
                            "type": "energise",
                            "coil": "output_coil"
                        }
                    ]
                },
                {
                    "name": "critical_rung",
                    "guard": {
                        "type": "and",
                        "operands": [
                            {
                                "type": "contact",
                                "name": "input_signal",
                                "contact_type": "NO"
                            },
                            {
                                "type": "contact",
                                "name": "system_ok",
                                "contact_type": "NO"
                            }
                        ]
                    },
                    "actions": [
                        {
                            "type": "energise",
                            "coil": "critical_output"
                        }
                    ]
                }
            ]
        }
    }"#;

    // Load program
    vm.load_program(ir_json).await?;
    println!("✓ Program loaded");

    // Track callback invocations
    let coil_change_count = Arc::new(AtomicU32::new(0));
    let cycle_complete_count = Arc::new(AtomicU32::new(0));
    let output_coil_changes = Arc::new(AtomicU32::new(0));

    // Register callback for specific coil
    let output_coil_count = output_coil_changes.clone();
    vm.on_coil_change("output_coil", move |name, old_val, new_val| {
        println!("  → Coil '{}' changed: {} → {}", name, old_val, new_val);
        output_coil_count.fetch_add(1, Ordering::Relaxed);
    })
    .await;

    // Register callback for any coil change
    let any_coil_count = coil_change_count.clone();
    vm.on_any_coil_change(move |name, old_val, new_val| {
        println!("  → Any coil changed: '{}' {} → {}", name, old_val, new_val);
        any_coil_count.fetch_add(1, Ordering::Relaxed);
    })
    .await;

    // Register callback for cycle completion
    let cycle_count = cycle_complete_count.clone();
    vm.on_cycle_complete(move |outputs| {
        println!("  → Cycle complete. Outputs: {:?}", outputs);
        cycle_count.fetch_add(1, Ordering::Relaxed);
    })
    .await;

    println!("\n--- First cycle ---");
    // Set input signal
    vm.set_signal("input_signal", true).await?;
    println!("✓ Signal 'input_signal' set to true");

    // Execute cycle - should trigger callbacks
    let outputs = vm.execute_cycle().await?;
    println!("✓ Cycle executed");

    println!("\n--- Second cycle (with system_ok) ---");
    // Set both signals
    vm.set_signal("system_ok", true).await?;
    println!("✓ Signal 'system_ok' set to true");

    // Execute cycle - should trigger more callbacks
    let outputs2 = vm.execute_cycle().await?;
    println!("✓ Cycle executed");

    println!("\n--- Third cycle (de-energise) ---");
    // Clear input signal
    vm.set_signal("input_signal", false).await?;
    println!("✓ Signal 'input_signal' set to false");

    // Execute cycle - should trigger callbacks for de-energised coils
    let outputs3 = vm.execute_cycle().await?;
    println!("✓ Cycle executed");

    // Print statistics
    println!("\n--- Callback Statistics ---");
    println!(
        "  Coil change callbacks: {}",
        coil_change_count.load(Ordering::Relaxed)
    );
    println!(
        "  output_coil specific callbacks: {}",
        output_coil_changes.load(Ordering::Relaxed)
    );
    println!(
        "  Cycle complete callbacks: {}",
        cycle_complete_count.load(Ordering::Relaxed)
    );

    Ok(())
}
