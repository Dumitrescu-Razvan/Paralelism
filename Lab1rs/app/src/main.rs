use rand::Rng;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

// Represents the shared state (primary and secondary variables)
struct SharedState {
    primary: RwLock<HashMap<String, i32>>,
    secondary: RwLock<HashMap<String, (Vec<String>, i32)>>, // (inputs, sum)
}

impl SharedState {
    fn new() -> Self {
        SharedState {
            primary: RwLock::new(HashMap::new()),
            secondary: RwLock::new(HashMap::new()),
        }
    }

    // Function to update a primary variable and propagate the changes to secondary variables
    fn update_primary(&self, var: &str, new_value: i32) {
        let mut primary = self.primary.write().unwrap(); // Write lock
        if let Some(old_value) = primary.get(var) {
            let delta = new_value - old_value;
            primary.insert(var.to_string(), new_value);

            // Update secondary variables that depend on this primary variable
            let mut secondary = self.secondary.write().unwrap(); // Write lock
            for (inputs, sum) in secondary.values_mut() {
                if inputs.contains(&var.to_string()) {
                    *sum += delta;
                }
            }
        }
    }

    // Function to add a primary variable
    fn add_primary(&self, var: String, value: i32) {
        let mut primary = self.primary.write().unwrap(); // Write lock
        primary.insert(var, value);
    }

    // Function to add a secondary variable
    fn add_secondary(&self, var: String, inputs: Vec<String>) {
        let mut secondary = self.secondary.write().unwrap(); // Write lock
        let primary = self.primary.read().unwrap(); // Read lock
        let sum: i32 = inputs.iter().map(|v| *primary.get(v).unwrap_or(&0)).sum();
        secondary.insert(var, (inputs, sum));
    }

    // Consistency check to verify all secondary variables are sums of their inputs
    fn check_consistency(&self) {
        let primary = self.primary.read().unwrap(); // Read lock
        let secondary = self.secondary.read().unwrap(); // Read lock
        for (var, (inputs, expected_sum)) in secondary.iter() {
            let actual_sum: i32 = inputs.iter().map(|v| *primary.get(v).unwrap_or(&0)).sum();
            assert_eq!(
                &actual_sum, expected_sum,
                "Inconsistent value for {}: expected {}, got {}",
                var, expected_sum, actual_sum
            );
        }
        println!("Consistency check passed!");
    }
}

fn main() {
    // Shared state between threads
    let shared_state = Arc::new(SharedState::new());

    // Initialize primary and secondary variables with random values
    let num_primary_vars = 100;
    let num_secondary_vars = 10000;
    let thread_number = 100;
    let mut rng = rand::thread_rng();

    // Generate random primary variables
    for i in 1..=num_primary_vars {
        let var_name = format!("x{}", i);
        let value = rng.gen_range(0..100); // Random value between 0 and 100
        shared_state.add_primary(var_name, value);
    }

    // Generate random secondary variables based on random primary inputs
    for i in 1..=num_secondary_vars {
        let var_name = format!("s{}", i);
        let num_inputs = rng.gen_range(2..=num_primary_vars / 2); // Random number of inputs (2 or 3)
        let mut inputs = Vec::new();
        while inputs.len() < num_inputs {
            let input = format!("x{}", rng.gen_range(1..=num_primary_vars));
            if !inputs.contains(&input) {
                inputs.push(input);
            }
        }
        shared_state.add_secondary(var_name, inputs);
    }

    print!("Initial state: ");
    shared_state.check_consistency();
    for (var, value) in shared_state.primary.read().unwrap().iter() {
        println!("{} = {}", var, value);
    }
    for (var, (inputs, sum)) in shared_state.secondary.read().unwrap().iter() {
        println!("{} = {} (inputs: {:?})", var, sum, inputs);
    }

    // Spawn a thread to randomly check consistency during updates
    let shared_state_consistency = Arc::clone(&shared_state);
    let check_handle = thread::spawn(move || {
        let mut thread_rng = rand::thread_rng();
        for _ in 0..5 {
            // Wait for a random period of time before checking consistency
            shared_state_consistency.check_consistency();
            thread::sleep(Duration::from_millis(thread_rng.gen_range(500..1000)));
        }
    });

    // Spawn multiple threads to update primary variables concurrently
    let mut handles = vec![];
    for i in 0..thread_number {
        let shared_state = Arc::clone(&shared_state);
        let handle = thread::spawn(move || {
            // Simulate some delay between updates
            let mut thread_rng = rand::thread_rng();
            thread::sleep(Duration::from_millis(thread_rng.gen_range(1000..5000)));
            
            let mut rng = rand::thread_rng();
            let rand_var = format!("x{}", rng.gen_range(1..=num_primary_vars));
            //let var = format!("x{}", (i % num_primary_vars) + 1);
            print!("Thread {}: Updating {}... \n", i, rand_var);
            let new_value = thread_rng.gen_range(0..100); // Random value between 0 and 100
            shared_state.update_primary(&rand_var, new_value);

        });
        handles.push(handle);
    }

    // Wait for all update threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    // Wait for the random consistency check thread to finish
    check_handle.join().unwrap();

    // Final consistency check at the end
    shared_state.check_consistency();
}
