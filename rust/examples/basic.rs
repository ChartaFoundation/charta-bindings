/// Basic example of using the Charta Rust SDK

use charta::{ChartaVM, Error};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("Charta Rust SDK - Basic Example");

    // Create VM instance
    let mut vm = ChartaVM::new();

    // Create a simple IR program
    let ir_json = r#"
    {
        "version": "0.1.0",
        "module": {
            "name": "example",
            "signals": [
                {"name": "input_signal"}
            ],
            "coils": [
                {"name": "output_coil"}
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
                }
            ]
        }
    }"#;

    // Load program
    vm.load_program(ir_json).await?;
    println!("✓ Program loaded");

    // Set input signal
    vm.set_signal("input_signal", true).await?;
    println!("✓ Signal 'input_signal' set to true");

    // Execute cycle
    let outputs = vm.execute_cycle().await?;
    println!("✓ Cycle executed");

    // Check output
    if let Some(&true) = outputs.get("output_coil") {
        println!("✓ Coil 'output_coil' is energised");
    } else {
        println!("✗ Coil 'output_coil' is not energised");
    }

    Ok(())
}
