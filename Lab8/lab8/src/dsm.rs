use mpi::traits::*;
use std::collections::{HashMap, HashSet};

use crate::variable::Variable;

pub struct DSM {
    pub local_variables: Vec<Variable>,
    pub subscriptions: HashMap<String, HashSet<usize>>, // Variable -> Subscribers (process ranks)
    pub rank: usize, // Rank of this MPI process
    pub size: usize, // Total number of MPI processes
}

impl DSM {
    /// Create a new DSM instance
    pub fn new(rank: usize, size: usize) -> Self {
        DSM {
            local_variables: Vec::new(),
            subscriptions: HashMap::new(),
            rank,
            size,
        }
    }

    /// Add a local variable
    pub fn add_variable(&mut self, var : Variable) {
        self.local_variables.push(var);
    }

    /// Write to a variable (local or distributed)
    pub fn write_variable(&mut self, name: &str, value: i32, world: &mpi::topology::SimpleCommunicator) {
        if let Some(var) = self.local_variables.iter_mut().find(|v| v.get_name() == name) {
            var.set_value(value);
            self.notify_subscribers(name, value, world);
        } else {
            // Variable not found locally, send to coordinator
            let serialized = bincode::serialize(&(name.to_string(), value)).unwrap();
            world.process_at_rank(0).send(&serialized);
        }
    }

    /// Notify subscribers about a variable update
    pub fn notify_subscribers(&self, name: &str, value: i32, world: &mpi::topology::SimpleCommunicator) {
        if let Some(subscribers) = self.subscriptions.get(name) {
            let serialized = bincode::serialize(&(name.to_string(), value)).unwrap();
            for &subscriber_rank in subscribers {
                world.process_at_rank(subscriber_rank as i32).send(&serialized);
            }
        }
    }

    pub fn compare_and_exchange(&mut self, name: &str, expected: i32, new: i32, world: &mpi::topology::SimpleCommunicator) -> bool {
        if let Some(var) = self.local_variables.iter_mut().find(|v| v.get_name() == name) {
            let current = var.get_value();
            if current == expected {
                var.set_value(new);
                self.notify_subscribers(name, new, world);
                true
            } else {
                false
            }
        } else {
            // Variable not found locally, send to coordinator
            let serialized = bincode::serialize(&(name.to_string(), expected, new)).unwrap();
            world.process_at_rank(0).send(&serialized);
            false
        }
    }
}
