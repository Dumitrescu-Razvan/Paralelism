use std::sync::{Arc, Condvar, Mutex};
use std::thread;

struct SharedData {
    current_product: i32,
    ready: bool,
    done: bool,
}

fn producer(vec1: Vec<i32>, vec2: Vec<i32>, data: Arc<(Mutex<SharedData>, Condvar)>) {
    let (lock, cvar) = &*data;

    for i in 0..vec1.len() {
        let mut shared_data = lock.lock().unwrap();
        shared_data.current_product = vec1[i] * vec2[i]; // Compute the product
        shared_data.ready = true;
        cvar.notify_one(); // Notify consumer that the product is ready
        shared_data = cvar
            .wait_while(shared_data, |data| data.ready)
            .unwrap(); // Wait until consumer processes it
    }

    // Signal that production is done
    let mut shared_data = lock.lock().unwrap();
    shared_data.done = true;
    cvar.notify_one(); // Notify consumer that production is complete
}

fn consumer(data: Arc<(Mutex<SharedData>, Condvar)>, result: Arc<Mutex<i32>>) {
    let (lock, cvar) = &*data;

    loop {
        let mut shared_data = lock.lock().unwrap();
        shared_data = cvar
            .wait_while(shared_data, |data| !data.ready && !data.done)
            .unwrap(); // Wait until product is ready or done

        if shared_data.done && !shared_data.ready {
            break; // Exit if producer is done
        }

        *result.lock().unwrap() += shared_data.current_product; // Accumulate the scalar product
        shared_data.ready = false;
        cvar.notify_one(); // Notify producer to produce next product
    }
}

fn scalar_product_basic (vec1: Vec<i32>, vec2: Vec<i32>) -> i32 {
    let mut result = 0;
    for i in 0..vec1.len() {
        result += vec1[i] * vec2[i];
    }
    result
}

fn main() {
    // let vec1 = vec![1, 2, 3, 4, 5];
    // let vec2 = vec![6, 7, 8, 9, 10];
    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();

    let vec_size = 100;
    for _i in 0..vec_size{
        let mut val = rand::random::<i32>() % 10;
        vec1.push(val);
        val = rand::random::<i32>() % 10;
        vec2.push(val);
    }

    // Initialize shared data
    let shared_data = Arc::new((Mutex::new(SharedData {
        current_product: 0,
        ready: false,
        done: false,
    }), Condvar::new()));

    // Shared result protected by a mutex
    let result = Arc::new(Mutex::new(0));

    // Clone the shared data for the consumer thread
    let shared_data_consumer = Arc::clone(&shared_data);
    let result_consumer = Arc::clone(&result);

    // Spawn producer and consumer threads
    let vec1_clone = vec1.clone();
    let vec2_clone = vec2.clone();

    let producer_thread = thread::spawn(move || {
        producer(vec1, vec2, shared_data);
    });

    let consumer_thread = thread::spawn(move || {
        consumer(shared_data_consumer, result_consumer);
    });

    // Wait for both threads to complete
    producer_thread.join().unwrap();
    consumer_thread.join().unwrap();

    // Get the final scalar product
    let scalar_product = *result.lock().unwrap();
    assert!(scalar_product == scalar_product_basic(vec1_clone, vec2_clone));
    println!("Scalar product: {}", scalar_product);
}
