use std::collections::HashMap;

pub(crate) struct DSM {
    variables: HashMap<String, i32>,
    subscribers: HashMap<String, Vec<i32>>,
}

impl DSM {
    pub(crate) fn new() -> DSM {
        DSM {
            variables: HashMap::new(),
            subscribers: HashMap::new(),
        }
    }

    pub(crate) fn add_variable(&mut self, name: String, value: i32) {
        self.variables.insert(name, value);
    }

    pub(crate) fn get_variable(&self, name: String) -> i32 {
        match self.variables.get(&name) {
            Some(value) => *value,
            None => 0,
        }
    }

    pub(crate) fn subscribe(&mut self, name: String, rank: i32) {
        match self.subscribers.get_mut(&name) {
            Some(subscribers) => subscribers.push(rank),
            None => {
                self.subscribers.insert(name, vec![rank]);
            }
        }
    }

    pub(crate) fn notify(&self, name: String, value: i32) {
        match self.subscribers.get(&name) {
            Some(subscribers) => {
                for rank in subscribers {
                    println!("Rank {} received notification: {} = {}", rank, name, value);
                }
            }
            None => {}
        }
    }
    
}