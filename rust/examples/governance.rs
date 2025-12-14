/// Example demonstrating governance patterns with the Charta Rust SDK

use charta::{ChartaVM, Error};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("Charta Rust SDK - Governance Pattern Example");

    // Create VM instance
    let mut vm = ChartaVM::new();

    // Create a governance-gated program
    let ir_json = r#"
    {
        "version": "0.1.0",
        "module": {
            "name": "governance_example",
            "signals": [
                {"name": "operation_requested"},
                {"name": "governance_ok"},
                {"name": "system_ok"},
                {"name": "compliance_ok"}
            ],
            "coils": [
                {"name": "allow_operation"},
                {"name": "governance_ok"}
            ],
            "rungs": [
                {
                    "name": "governance_interlock",
                    "guard": {
                        "type": "and",
                        "operands": [
                            {
                                "type": "contact",
                                "name": "compliance_ok",
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
                            "coil": "governance_ok"
                        }
                    ]
                },
                {
                    "name": "operation_gate",
                    "guard": {
                        "type": "and",
                        "operands": [
                            {
                                "type": "contact",
                                "name": "operation_requested",
                                "contact_type": "NO"
                            },
                            {
                                "type": "contact",
                                "name": "governance_ok",
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
                            "coil": "allow_operation"
                        }
                    ]
                }
            ]
        }
    }"#;

    // Load program
    vm.load_program(ir_json).await?;
    println!("✓ Program loaded");

    println!("\n--- Scenario 1: All conditions met ---");
    vm.set_signal("operation_requested", true).await?;
    vm.set_signal("system_ok", true).await?;
    vm.set_signal("compliance_ok", true).await?;

    let outputs = vm.execute_cycle().await?;
    println!("  Operation requested: ✓");
    println!("  System OK: ✓");
    println!("  Compliance OK: ✓");
    if outputs.get("allow_operation") == Some(&true) {
        println!("  → Operation ALLOWED ✓");
    } else {
        println!("  → Operation BLOCKED ✗");
    }

    println!("\n--- Scenario 2: Governance not OK ---");
    vm.set_signal("compliance_ok", false).await?;

    let outputs = vm.execute_cycle().await?;
    println!("  Operation requested: ✓");
    println!("  System OK: ✓");
    println!("  Compliance OK: ✗");
    if outputs.get("allow_operation") == Some(&true) {
        println!("  → Operation ALLOWED ✓");
    } else {
        println!("  → Operation BLOCKED ✓ (expected)");
    }

    println!("\n--- Scenario 3: System not OK ---");
    vm.set_signal("compliance_ok", true).await?;
    vm.set_signal("system_ok", false).await?;

    let outputs = vm.execute_cycle().await?;
    println!("  Operation requested: ✓");
    println!("  System OK: ✗");
    println!("  Compliance OK: ✓");
    if outputs.get("allow_operation") == Some(&true) {
        println!("  → Operation ALLOWED ✓");
    } else {
        println!("  → Operation BLOCKED ✓ (expected)");
    }

    println!("\n--- Final state ---");
    let all_coils = vm.get_all_coils().await?;
    for (name, value) in all_coils {
        println!("  {}: {}", name, value);
    }

    Ok(())
}
