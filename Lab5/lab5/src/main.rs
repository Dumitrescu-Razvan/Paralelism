use std::time::Instant;
use rayon::prelude::*;

fn multiply_polynomials_naive(a: &[i128], b: &[i128]) -> Vec<i128> {
    let mut result = vec![0; a.len() + b.len() - 1];
    for i in 0..a.len() {
        for j in 0..b.len() {
            result[i + j] += a[i] * b[j];
        }
    }
    result
}

fn multiply_polynomials_naive_parallel(a: &[i128], b: &[i128]) -> Vec<i128> {
    let mut result = vec![0; a.len() + b.len() - 1];
    result.par_iter_mut().enumerate().for_each(|(k, res)| {
        for i in 0..a.len() {
            if k >= i && k - i < b.len() {
                *res += a[i] * b[k - i];
            }
        }
    });
    result
}

fn karatsuba(a: &[i128], b: &[i128]) -> Vec<i128> {
    let n = a.len();
    if n <= 1 {
        return vec![a[0] * b[0]];
    }
    let k = n / 2;

    let a1 = &a[..k];
    let a2 = &a[k..];
    let b1 = &b[..k];
    let b2 = &b[k..];

    let a1b1 = karatsuba(a1, b1);
    let a2b2 = karatsuba(a2, b2);

    let mut a1a2 = vec![0; k];
    let mut b1b2 = vec![0; k];
    for i in 0..k {
        a1a2[i] = a1[i] + a2[i];
        b1b2[i] = b1[i] + b2[i];
    }

    let a1a2b1b2 = karatsuba(&a1a2, &b1b2);

    let mut result = vec![0; 2 * n - 1];
    for i in 0..a1b1.len() {
        result[i] += a1b1[i];
        result[i + n] += a2b2[i];
        result[i + k] += a1a2b1b2[i] - a1b1[i] - a2b2[i];
    }
    result
}

fn karatsuba_parallel(a: &[i128], b: &[i128]) -> Vec<i128> {
    let n = a.len();
    if n <= 1 {
        return vec![a[0] * b[0]];
    }
    let k = n / 2;

    let (a1, a2) = a.split_at(k);
    let (b1, b2) = b.split_at(k);

    let (a1b1, (a2b2, a1a2b1b2)) = rayon::join(
        || karatsuba_parallel(a1, b1),
        || rayon::join(
            || karatsuba_parallel(a2, b2),
            || {
                let a1a2: Vec<i128> = a1.iter().zip(a2.iter()).map(|(&x, &y)| x + y).collect();
                let b1b2: Vec<i128> = b1.iter().zip(b2.iter()).map(|(&x, &y)| x + y).collect();
                karatsuba_parallel(&a1a2, &b1b2)
            }
        )
    );

    let mut result = vec![0; 2 * n - 1];
    for i in 0..a1b1.len() {
        result[i] += a1b1[i];
        result[i + n] += a2b2[i];
        result[i + k] += a1a2b1b2[i] - a1b1[i] - a2b2[i];
    }
    result
}

fn main() {

    let mut a = vec![];
    let mut b = vec![];

    for _ in 0..100 {
        a.push(rand::random::<i128>() % 10000);
        b.push(rand::random::<i128>() % 10000);
    }

    let start = Instant::now();
    let result_naive = multiply_polynomials_naive(&a, &b);
    let duration_naive = start.elapsed();
    println!("Naive: {:?}, Time: {:?}", result_naive, duration_naive);

    let start = Instant::now();
    let result_naive_parallel = multiply_polynomials_naive_parallel(&a, &b);
    let duration_naive_parallel = start.elapsed();
    println!("Naive Parallel: {:?}, Time: {:?}", result_naive_parallel, duration_naive_parallel);

    let start = Instant::now();
    let result_karatsuba = karatsuba(&a, &b);
    let duration_karatsuba = start.elapsed();
    println!("Karatsuba: {:?}, Time: {:?}", result_karatsuba, duration_karatsuba);

    let start = Instant::now();
    let result_karatsuba_parallel = karatsuba_parallel(&a, &b);
    let duration_karatsuba_parallel = start.elapsed();
    println!("Karatsuba Parallel: {:?}, Time: {:?}", result_karatsuba_parallel
    , duration_karatsuba_parallel);
}