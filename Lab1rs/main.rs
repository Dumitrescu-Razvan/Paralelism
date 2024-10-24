extern crate rand; 

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use rand::prelude::*;

// Structure to represent the variables and dependencies
struct Variable {
    value: i32,             // The current value of the variable
    dependents: Vec<usize>, // List of indices of secondary variables that depend on this variable
}

// Structure to represent the system of variables
struct VariableSystem {
    variables: Vec<Mutex<Variable>>, // Mutex-protected variables
    secondary_relations: HashMap<usize, Vec<usize>>, // Secondary variable dependencies: secondary -> input variables
}

impl VariableSystem {
    // Create a new system with the given number of variables
    fn new(num_vars: usize) -> VariableSystem {
        let mut variables = Vec::new();
        for _ in 0..num_vars {
            variables.push(Mutex::new(Variable {
                value: rand::random::<i32>() % 100, // Random initial value
                dependents: Vec::new(),
            }));
        }
        VariableSystem {
            variables,
            secondary_relations: HashMap::new(),
        }
    }

    // Set up a secondary variable that depends on a list of input variables
    fn add_secondary_variable(&mut self, sec_var_idx: usize, input_vars: Vec<usize>) {
        // Lock the input variables and register them as dependents of the secondary variable
        for &input_var_idx in &input_vars {
            let mut input_var = self.variables[input_var_idx].lock().unwrap();
            input_var.dependents.push(sec_var_idx);
        }
        // Store the relationship
        self.secondary_relations.insert(sec_var_idx, input_vars);
    }

    // Atomically update a primary variable and propagate the changes to all dependent variables
    fn update_primary(&self, var_idx: usize, new_value: i32) {
        println!("Updating primary variable {} to {}", var_idx, new_value);

        // Lock the primary variable and get the old value
        let mut primary_var = self.variables[var_idx].lock().unwrap();
        let old_value = primary_var.value;
        let diff = new_value - old_value;
        primary_var.value = new_value;

        // Propagate the difference to dependent secondary variables
        for &dependent_idx in &primary_var.dependents {
            self.propagate_update(dependent_idx, diff);
        }
    }

    // Recursively propagate updates through secondary variables
    fn propagate_update(&self, sec_var_idx: usize, diff: i32) {
        let mut sec_var = self.variables[sec_var_idx].lock().unwrap();
        sec_var.value += diff;

        // If this secondary variable depends on others, propagate further
        if let Some(deps) = self.secondary_relations.get(&sec_var_idx) {
            for &dep_idx in deps {
                self.propagate_update(dep_idx, diff);
            }
        }
    }

    // Perform a consistency check to ensure all secondary variables have the correct values
    fn check_consistency(&self) {
        for (&sec_var_idx, deps) in &self.secondary_relations {
            let mut expected_sum = 0;
            for &dep_idx in deps {
                let dep_var = self.variables[dep_idx].lock().unwrap();
                expected_sum += dep_var.value;
            }

            let sec_var = self.variables[sec_var_idx].lock().unwrap();
            if sec_var.value != expected_sum {
                println!(
                    "Inconsistency found: secondary variable {} has value {}, expected {}",
                    sec_var_idx, sec_var.value, expected_sum
                );
            } else {
                println!("Secondary variable {} is consistent", sec_var_idx);
            }
        }
    }
}

fn main() {
    // Create a system with 5 variables (0-4)
    let var_sys = Arc::new(Mutex::new(VariableSystem::new(5)));

    for i in 0..5 {
        let sys = var_sys.lock().unwrap();
        println!("Variable {}: {}", i, sys.variables[i].lock().unwrap().value);
    }

    // Add secondary variables: 3 depends on 0 and 1, 4 depends on 2 and 3
    {
        let mut sys = var_sys.lock().unwrap();
        sys.add_secondary_variable(3, vec![0, 1]);
        sys.add_secondary_variable(4, vec![2, 3]);
    }

    println!("System initialized");

    // Launch threads to update primary variables
    let mut handles = Vec::new();
    for i in 0..3 {
        let sys_clone = Arc::clone(&var_sys);
        handles.push(thread::spawn(move || {
            // Randomly update primary variables
            let sys = sys_clone.lock().unwrap();
            sys.update_primary(i, i as i32 * 10); // For simplicity, new values are multiples of 10
        }));
    }

    println!("Threads launched");

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Threads finished");

    // Perform a final consistency check
    let sys = var_sys.lock().unwrap();
    sys.check_consistency();

    println!("Consistency check finished");
}
