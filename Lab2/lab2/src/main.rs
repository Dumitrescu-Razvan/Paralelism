use std:: thread;
use std::sync::{Arc, Mutex, mpsc};



fn prod_fn(vec1: Vec<i32>, vec2: Vec<i32>, tx: mpsc::Sender<i32>) {
    for i in 0..vec1.len() {
        let prod = vec1[i] * vec2[i];
        tx.send(prod).unwrap();
    }

}


fn cons_fn(scalar_product: Arc<Mutex<i32>>, rx: mpsc::Receiver<i32>) {
    for product in rx {
        let mut result = scalar_product.lock().unwrap();
        *result += product;
    }

}

fn main() {
    
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];
    
    
    let result_prod = Arc::new(Mutex::new(0));
    let result_cons = Arc::clone(&result_prod);

    let (tx, rx) = mpsc::channel();

    let producer = thread::spawn(move || {
        prod_fn(vec1, vec2, tx);
    });

    let consumer = thread::spawn(move || {
        cons_fn(result_cons,rx);
    });

    producer.join().unwrap();
    consumer.join().unwrap();

    let result = result_prod.lock().unwrap();
    println!("The scalar product is: {}", *result);

}
