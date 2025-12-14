/// Integration tests for Charta Rust SDK

use charta::{ChartaVM, Error};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

#[tokio::test]
async fn test_basic_vm_operations() -> Result<(), Error> {
    let mut vm = ChartaVM::new();

    let ir_json = r#"
    {
        "version": "0.1.0",
        "module": {
            "name": "test",
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

    vm.load_program(ir_json).await?;

    // Test signal setting
    vm.set_signal("input_signal", true).await?;
    assert_eq!(vm.get_signal("input_signal").await?, Some(true));

    // Test cycle execution
    let outputs = vm.execute_cycle().await?;
    assert_eq!(outputs.get("output_coil"), Some(&true));

    // Test coil reading
    assert_eq!(vm.get_coil("output_coil").await?, Some(true));

    Ok(())
}

#[tokio::test]
async fn test_signal_and_coil_management() -> Result<(), Error> {
    let mut vm = ChartaVM::new();

    let ir_json = r#"
    {
        "version": "0.1.0",
        "module": {
            "name": "test",
            "signals": [
                {"name": "signal1"},
                {"name": "signal2"}
            ],
            "coils": [
                {"name": "coil1"},
                {"name": "coil2"}
            ],
            "rungs": []
        }
    }"#;

    vm.load_program(ir_json).await?;

    // Test signal names
    let signal_names = vm.signal_names().await?;
    assert_eq!(signal_names.len(), 2);
    assert!(signal_names.contains(&"signal1".to_string()));
    assert!(signal_names.contains(&"signal2".to_string()));

    // Test coil names
    let coil_names = vm.coil_names().await?;
    assert_eq!(coil_names.len(), 2);
    assert!(coil_names.contains(&"coil1".to_string()));
    assert!(coil_names.contains(&"coil2".to_string()));

    // Test setting and getting signals
    vm.set_signal("signal1", true).await?;
    vm.set_signal("signal2", false).await?;

    assert_eq!(vm.get_signal("signal1").await?, Some(true));
    assert_eq!(vm.get_signal("signal2").await?, Some(false));

    // Test getting all signals
    let all_signals = vm.get_all_signals().await?;
    assert_eq!(all_signals.get("signal1"), Some(&true));
    assert_eq!(all_signals.get("signal2"), Some(&false));

    // Test setting and getting coils
    vm.set_coil("coil1", true).await?;
    vm.set_coil("coil2", false).await?;

    assert_eq!(vm.get_coil("coil1").await?, Some(true));
    assert_eq!(vm.get_coil("coil2").await?, Some(false));

    // Test getting all coils
    let all_coils = vm.get_all_coils().await?;
    assert_eq!(all_coils.get("coil1"), Some(&true));
    assert_eq!(all_coils.get("coil2"), Some(&false));

    Ok(())
}

#[tokio::test]
async fn test_execute_cycle_with_inputs() -> Result<(), Error> {
    let mut vm = ChartaVM::new();

    let ir_json = r#"
    {
        "version": "0.1.0",
        "module": {
            "name": "test",
            "signals": [
                {"name": "input_a"},
                {"name": "input_b"}
            ],
            "coils": [
                {"name": "output"}
            ],
            "rungs": [
                {
                    "name": "test_rung",
                    "guard": {
                        "type": "and",
                        "left": {
                            "type": "contact",
                            "name": "input_a",
                            "contact_type": "NO"
                        },
                        "right": {
                            "type": "contact",
                            "name": "input_b",
                            "contact_type": "NO"
                        }
                    },
                    "actions": [
                        {
                            "type": "energise",
                            "coil": "output"
                        }
                    ]
                }
            ]
        }
    }"#;

    vm.load_program(ir_json).await?;

    // Test with both inputs true
    let mut inputs = HashMap::new();
    inputs.insert("input_a".to_string(), true);
    inputs.insert("input_b".to_string(), true);

    let outputs = vm.execute_cycle_with_inputs(inputs).await?;
    assert_eq!(outputs.get("output"), Some(&true));

    // Test with one input false
    let mut inputs2 = HashMap::new();
    inputs2.insert("input_a".to_string(), true);
    inputs2.insert("input_b".to_string(), false);

    let outputs2 = vm.execute_cycle_with_inputs(inputs2).await?;
    assert_eq!(outputs2.get("output"), Some(&false));

    Ok(())
}

#[tokio::test]
async fn test_coil_change_callbacks() -> Result<(), Error> {
    let mut vm = ChartaVM::new();

    let ir_json = r#"
    {
        "version": "0.1.0",
        "module": {
            "name": "test",
            "signals": [
                {"name": "input"}
            ],
            "coils": [
                {"name": "output"}
            ],
            "rungs": [
                {
                    "name": "test_rung",
                    "guard": {
                        "type": "contact",
                        "name": "input",
                        "contact_type": "NO"
                    },
                    "actions": [
                        {
                            "type": "energise",
                            "coil": "output"
                        }
                    ]
                }
            ]
        }
    }"#;

    vm.load_program(ir_json).await?;

    // Track callback invocations
    let callback_count = Arc::new(AtomicU32::new(0));
    let callback_count_clone = callback_count.clone();

    // Register callback
    vm.on_coil_change("output", move |name, old_val, new_val| {
        assert_eq!(name, "output");
        assert_eq!(old_val, false);
        assert_eq!(new_val, true);
        callback_count_clone.fetch_add(1, Ordering::Relaxed);
    })
    .await;

    // Execute cycle - should trigger callback
    vm.set_signal("input", true).await?;
    vm.execute_cycle().await?;

    // Verify callback was called
    assert_eq!(callback_count.load(Ordering::Relaxed), 1);

    Ok(())
}

#[tokio::test]
async fn test_any_coil_change_callbacks() -> Result<(), Error> {
    let mut vm = ChartaVM::new();

    let ir_json = r#"
    {
        "version": "0.1.0",
        "module": {
            "name": "test",
            "signals": [
                {"name": "input1"},
                {"name": "input2"}
            ],
            "coils": [
                {"name": "output1"},
                {"name": "output2"}
            ],
            "rungs": [
                {
                    "name": "rung1",
                    "guard": {
                        "type": "contact",
                        "name": "input1",
                        "contact_type": "NO"
                    },
                    "actions": [
                        {
                            "type": "energise",
                            "coil": "output1"
                        }
                    ]
                },
                {
                    "name": "rung2",
                    "guard": {
                        "type": "contact",
                        "name": "input2",
                        "contact_type": "NO"
                    },
                    "actions": [
                        {
                            "type": "energise",
                            "coil": "output2"
                        }
                    ]
                }
            ]
        }
    }"#;

    vm.load_program(ir_json).await?;

    // Track callback invocations
    let callback_count = Arc::new(AtomicU32::new(0));
    let callback_count_clone = callback_count.clone();

    // Register callback for any coil
    vm.on_any_coil_change(move |_name, _old_val, _new_val| {
        callback_count_clone.fetch_add(1, Ordering::Relaxed);
    })
    .await;

    // Execute cycle with both inputs - should trigger callback twice
    vm.set_signal("input1", true).await?;
    vm.set_signal("input2", true).await?;
    vm.execute_cycle().await?;

    // Verify callback was called for both coils
    assert_eq!(callback_count.load(Ordering::Relaxed), 2);

    Ok(())
}

#[tokio::test]
async fn test_cycle_complete_callbacks() -> Result<(), Error> {
    let mut vm = ChartaVM::new();

    let ir_json = r#"
    {
        "version": "0.1.0",
        "module": {
            "name": "test",
            "signals": [
                {"name": "input"}
            ],
            "coils": [
                {"name": "output"}
            ],
            "rungs": [
                {
                    "name": "test_rung",
                    "guard": {
                        "type": "contact",
                        "name": "input",
                        "contact_type": "NO"
                    },
                    "actions": [
                        {
                            "type": "energise",
                            "coil": "output"
                        }
                    ]
                }
            ]
        }
    }"#;

    vm.load_program(ir_json).await?;

    // Track callback invocations
    let callback_count = Arc::new(AtomicU32::new(0));
    let callback_count_clone = callback_count.clone();

    // Register callback
    vm.on_cycle_complete(move |outputs| {
        assert!(outputs.contains_key("output"));
        callback_count_clone.fetch_add(1, Ordering::Relaxed);
    })
    .await;

    // Execute cycles
    vm.set_signal("input", true).await?;
    vm.execute_cycle().await?;
    vm.execute_cycle().await?;
    vm.execute_cycle().await?;

    // Verify callback was called for each cycle
    assert_eq!(callback_count.load(Ordering::Relaxed), 3);

    Ok(())
}

#[tokio::test]
async fn test_error_handling() -> Result<(), Error> {
    let mut vm = ChartaVM::new();

    // Test loading invalid IR
    let result = vm.load_program("invalid json").await;
    assert!(result.is_err());

    // Test getting non-existent signal/coil
    let ir_json = r#"
    {
        "version": "0.1.0",
        "module": {
            "name": "test",
            "signals": [],
            "coils": [],
            "rungs": []
        }
    }"#;

    vm.load_program(ir_json).await?;

    // Non-existent signal/coil should return None, not error
    assert_eq!(vm.get_signal("nonexistent").await?, None);
    assert_eq!(vm.get_coil("nonexistent").await?, None);

    Ok(())
}
