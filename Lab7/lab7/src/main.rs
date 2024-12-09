use mpi::traits::*;
use mpi::collective::SystemOperation;

/// Function to multiply two polynomials using the regular O(n^2) method
fn multiply_regular(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut result = vec![0; a.len() + b.len() - 1];
    for i in 0..a.len() {
        for j in 0..b.len() {
            result[i + j] += a[i] * b[j];
        }
    }
    result
}

/// Function to multiply two polynomials using the Karatsuba method
fn karatsuba(a: &[i32], b: &[i32]) -> Vec<i32> {
    let n = a.len();
    if n == 1 {
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

fn main() {
    let universe = mpi::initialize().unwrap();
    let world = universe.world();
    let size = world.size() as usize;
    let rank = world.rank() as usize;

    // Example polynomials
    let poly_a = vec![1, 2, 3, 4]; // 1 + 2x + 3x^2 + 4x^3
    let poly_b = vec![4, 3, 2, 1]; // 4 + 3x + 2x^2 + x^3

    // Split work among nodes
    let chunk_size = poly_a.len() / size;
    let start = rank * chunk_size;
    let end = if rank == size - 1 {
        poly_a.len()
    } else {
        (rank + 1) * chunk_size
    };

    let mut a_chunk = poly_a[start..end].to_vec();
    let mut b_chunk = poly_b.to_vec();

    // Pad chunks to the same length if necessary
    while a_chunk.len() < b_chunk.len() {
        a_chunk.push(0);
    }
    while b_chunk.len() < a_chunk.len() {
        b_chunk.push(0);
    }

    // Compute the result for the assigned chunk
    let mut local_result = if rank % 2 == 0 {
        multiply_regular(&a_chunk, &b_chunk)
    } else {
        karatsuba(&a_chunk, &b_chunk)
    };

    //Shift the result to the right by "rank" positions
    for _ in 0..rank {
        local_result.insert(0, 0);
        local_result.pop();
    } 
    println!("Rank: {}, Result: {:?}", rank, local_result);

    // Prepare global result buffer
    let mut global_result = vec![0; poly_a.len() + poly_b.len() - 1];

    // Reduce the results into the root process
    world.all_reduce_into(&local_result[..], &mut global_result[..], SystemOperation::sum());

    if rank == 0 {
        println!("Final Result: {:?}", global_result);
    }
}