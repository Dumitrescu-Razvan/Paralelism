/*
Solve the problem below:

Given a directed graph, find a Hamiltonean cycle, if one exists. Use multiple threads to parallelize the search. Important The search should start from a fixed vertex (no need to take each vertex as the starting point), however, the splitting of the work between threads should happen at several levels, for all possible choices among the neighbors of each current vertex.

*/

use std::sync::mpsc;
use std::thread;

fn hamiltonian_cycle(graph: Vec<Vec<i32>>) -> Vec<usize> {
    let n = graph.len();
    let mut cycle = vec![];
    let visited = vec![false; n];

    let mut handles= Vec::new();

    let (tx, rx) = mpsc::channel();

    for i in 0..n {
        let tx = tx.clone();
        let graph = graph.clone();
        let visited = visited.clone();
        let cycle = cycle.clone();
        handles.push(thread::spawn(move || {
            let mut visited = visited;
            let mut cycle = cycle;
            visited[i] = true;
            cycle.push(i);
            hamiltonian_cycle_helper(i, &graph, &mut visited, &mut cycle, &tx);
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    for _ in 0..n {
        let c = rx.recv().unwrap();
        if c.len() > cycle.len() {
            cycle = c;
        }
    }

    cycle
}

fn hamiltonian_cycle_helper(v: usize, graph: &Vec<Vec<i32>>, visited: &mut Vec<bool>, cycle: &mut Vec<usize>, tx: &mpsc::Sender<Vec<usize>>) {
    if cycle.len() == graph.len() {
        tx.send(cycle.clone()).unwrap();
        return;
    }

    let mut handles = Vec::new();

    for i in 0..graph.len() {
        if graph[v][i] == 1 && !visited[i] {
            let mut visited = visited.clone();
            let mut cycle = cycle.clone();
            let tx = tx.clone();
            let graph = graph.clone();
            visited[i] = true;
            cycle.push(i);
            handles.push(thread::spawn(move || {
                hamiltonian_cycle_helper(i, &graph, &mut visited, &mut cycle, &tx);
            }));
        }
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

fn main() {

    let graph = vec![
        vec![0, 1, 0, 1, 0],
        vec![1, 0, 1, 1, 1],
        vec![0, 1, 0, 0, 1],
        vec![1, 1, 0, 0, 1],
        vec![0, 1, 1, 1, 0],
    ];

    let cycle = hamiltonian_cycle(graph);

    for i in cycle {
        print!("{} ", i);
    }
    println!();


}
