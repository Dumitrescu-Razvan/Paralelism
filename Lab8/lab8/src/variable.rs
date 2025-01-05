use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Variable{
    name : Arc<Mutex<String>>,
    value :Arc<Mutex<i32>>,
    subscribers: Arc<Mutex<Vec<i32>>>,
}

impl Variable{
    pub fn new() -> Self{
        Variable{
            name : Arc::new(Mutex::new("".to_string())),
            value : Arc::new(Mutex::new(0)),
            subscribers: Arc::new(Mutex::new(Vec::new()))
        }
    }

    pub fn get_value(&self) -> i32{
        *self.value.lock().unwrap()
    }

    pub fn set_value(&self, new_value : i32) {
        *self.value.lock().unwrap() = new_value;
    }

    pub fn get_name(&self) -> String{
        self.name.lock().unwrap().clone()
    }

    pub fn set_name(&self, new_name : String){
        *self.name.lock().unwrap() = new_name;
    }

    pub fn get_subscribers(&self) -> Vec<i32>{
        self.subscribers.lock().unwrap().clone()
    }

    pub fn add_subscriber(&self, rank : i32){
        self.subscribers.lock().unwrap().push(rank);
    }
}