/// Example demonstrating multiple cycle execution with state tracking

use charta::{ChartaVM, Error};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("Charta Rust SDK - Multiple Cycles Example");

    // Create VM instance
    let mut vm = ChartaVM::new();

    // Create a program with latching coil
    let ir_json = r#"
    {
        "version": "0.1.0",
        "module": {
            "name": "multiple_cycles",
            "signals": [
                {"name": "start"},
                {"name": "stop"}
            ],
            "coils": [
                {"name": "running", "latching": true},
                {"name": "status_light"}
            ],
            "rungs": [
                {
                    "name": "start_rung",
                    "guard": {
                        "type": "contact",
                        "name": "start",
                        "contact_type": "NO"
                    },
                    "actions": [
                        {
                            "type": "energise",
                            "coil": "running"
                        }
                    ]
                },
                {
                    "name": "stop_rung",
                    "guard": {
                        "type": "contact",
                        "name": "stop",
                        "contact_type": "NO"
                    },
                    "actions": [
                        {
                            "type": "de_energise",
                            "coil": "running"
                        }
                    ]
                },
                {
                    "name": "status_light_rung",
                    "guard": {
                        "type": "contact",
                        "name": "running",
                        "contact_type": "NO"
                    },
                    "actions": [
                        {
                            "type": "energise",
                            "coil": "status_light"
                        }
                    ]
                }
            ]
        }
    }"#;

    // Load program
    vm.load_program(ir_json).await?;
    println!("✓ Program loaded\n");

    // Track state across cycles
    let mut cycle = 1;

    println!("--- Cycle {}: Initial state ---", cycle);
    let coils = vm.get_all_coils().await?;
    println!("  Coils: {:?}", coils);
    cycle += 1;

    println!("\n--- Cycle {}: Start signal ---", cycle);
    vm.set_signal("start", true).await?;
    let outputs = vm.execute_cycle().await?;
    println!("  Outputs: {:?}", outputs);
    let coils = vm.get_all_coils().await?;
    println!("  All coils: {:?}", coils);
    cycle += 1;

    println!("\n--- Cycle {}: Start signal cleared (latching should maintain) ---", cycle);
    vm.set_signal("start", false).await?;
    let outputs = vm.execute_cycle().await?;
    println!("  Outputs: {:?}", outputs);
    let coils = vm.get_all_coils().await?;
    println!("  All coils: {:?}", coils);
    println!("  → 'running' should still be true (latching)");
    cycle += 1;

    println!("\n--- Cycle {}: Stop signal ---", cycle);
    vm.set_signal("stop", true).await?;
    let outputs = vm.execute_cycle().await?;
    println!("  Outputs: {:?}", outputs);
    let coils = vm.get_all_coils().await?;
    println!("  All coils: {:?}", coils);
    cycle += 1;

    println!("\n--- Cycle {}: Stop signal cleared ---", cycle);
    vm.set_signal("stop", false).await?;
    let outputs = vm.execute_cycle().await?;
    println!("  Outputs: {:?}", outputs);
    let coils = vm.get_all_coils().await?;
    println!("  All coils: {:?}", coils);

    println!("\n✓ Multiple cycles completed successfully");

    Ok(())
}
