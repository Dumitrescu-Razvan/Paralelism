use::std::thread;
use::std::sync::{Arc, Mutex};
use::threadpool::ThreadPool;

const TASKS: usize = 100;
const ROWS: usize = 1000;
const COLS: usize = 1000;


fn element(line : Vec<i32>, column : Vec<i32>) -> i32 {
    let mut result = 0;
    for i in 0..line.len() {
        result += line[i] * column[i];
    }
    result 
}

fn print_matrix(matrix : Vec<Vec<i32>>) {
    for i in 0..9 {
        for j in 0..9 {
            print!("{} ", matrix[i][j]);
        }
        print!("\n");
    }
}

fn check_result(matrix1 : Vec<Vec<i32>>, matrix2 : Vec<Vec<i32>>, result : Vec<Vec<i32>>) -> bool {
    for i in 0..9 {
        for j in 0..9 {
            if result[i][j] != element(matrix1[i].clone(), matrix2.iter().map(|x| x[j]).collect()) {
                return false;
            }
        }
    }
    true
}


fn main() {
    let matrix1 = Arc::new(Mutex::new(vec![vec![0; 1000]; 1000]));
    let matrix2 = Arc::new(Mutex::new(vec![vec![0; 1000]; 1000]));

    for i in 0..COLS {
        for j in 0..ROWS {
            matrix1.lock().unwrap()[i][j] = rand::random::<i32>() % 10;
            matrix2.lock().unwrap()[i][j] = rand::random::<i32>() % 10;
        }
    }

    print!("Matrix 1: \n");
    print_matrix(matrix1.lock().unwrap().clone());

    print!("Matrix 2: \n");
    print_matrix(matrix2.lock().unwrap().clone());

    let simple_thread_time: std::time::Instant = std::time::Instant::now();

    let restult_simple_thread = Arc::new(Mutex::new(vec![vec![0; 1000]; 1000]));
    let mut threads = vec![];
    for task_id in 0..TASKS {
        let matrix1 = matrix1.clone();
        let matrix2 = matrix2.clone();
        let result = restult_simple_thread.clone();
        threads.push(thread::spawn(move || {
            for i in 0..1000 {
                for j in (task_id..1000).step_by(TASKS) {
                    result.lock().unwrap()[i][j] = element(matrix1.lock().unwrap()[i].clone(), matrix2.lock().unwrap().iter().map(|x| x[j]).collect());
                }
            }
        }));
    }

    for t in threads {
        t.join().unwrap();
    }

    println!("Simple thread time: {:?}", simple_thread_time.elapsed());

    // assert!(check_result(matrix1.lock().unwrap().clone(), matrix2.lock().unwrap().clone(), restult_simple_thread.lock().unwrap().clone()));

    let thread_pool_time = std::time::Instant::now();


    let pool = ThreadPool::new(TASKS);
    let restult_thread_pool = Arc::new(Mutex::new(vec![vec![0; 1000]; 1000]));
    for task_id in 0..TASKS {
        let matrix1 = matrix1.clone();
        let matrix2 = matrix2.clone();
        let result = restult_thread_pool.clone();
        pool.execute(move || {
            for i in 0..1000 {
                for j in (task_id..1000).step_by(TASKS) {
                    result.lock().unwrap()[i][j] = element(matrix1.lock().unwrap()[i].clone(), matrix2.lock().unwrap().iter().map(|x| x[j]).collect());
                }
            }
        });
    }

    pool.join();

    println!("Thread pool time: {:?}", thread_pool_time.elapsed());

    // assert!(check_result(matrix1.lock().unwrap().clone(), matrix2.lock().unwrap().clone(), restult_thread_pool.lock().unwrap().clone()));  

}
