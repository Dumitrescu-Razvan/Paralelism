mod dsm;
mod variable;
use mpi::traits::*;
use dsm::DSM;
use std::{collections::HashSet, thread};
fn main() {
    let universe = mpi::initialize().unwrap();
    let world = universe.world();
    let rank = world.rank() as usize;
    let size = world.size() as usize;

    let mut dsm = DSM::new(rank, size);

    if rank == 0 {
        let var = variable::Variable::new();
        var.set_name("x".to_string());
        var.set_value(42);
        var.add_subscriber(1);
        var.add_subscriber(2);
        dsm.add_variable(var);
        let var2 = variable::Variable::new();
        var2.set_name("x1".to_string());
        var2.set_value(42);
        var2.add_subscriber(1);
        var2.add_subscriber(2);
        dsm.add_variable(var2);
        println!("Rank 0");
    }
    else if rank == 1 {
        let var = variable::Variable::new();
        var.set_name("y".to_string());
        var.set_value(0);
        var.add_subscriber(0);
        dsm.add_variable(var);
        println!("Rank 1");
    }
    else if rank == 2 {
        let var = variable::Variable::new();
        var.set_name("z".to_string());
        var.set_value(0);
        var.add_subscriber(0);
        var.add_subscriber(3);
        dsm.add_variable(var);
        println!("Rank 2");
    }
    else if rank == 3 {
        let var = variable::Variable::new();
        var.set_name("w".to_string());
        var.set_value(0);
        var.add_subscriber(2);

        dsm.add_variable(var);
        println!("Rank 3");
        
    }

    world.barrier();

    //simulate some changes
    if rank == 0 {
        dsm.write_variable("x", 43, &world);
        
        println!("Rank 0: x = {}", dsm.local_variables[0].get_value());
    }
    else if rank == 1 {
        dsm.write_variable("y", 1, &world);
        println!("Rank 1: y = {}", dsm.local_variables[0].get_value());
    }
    else if rank == 2 {
        dsm.write_variable("z", 2, &world);
        println!("Rank 2: z = {}", dsm.local_variables[0].get_value());
    }
    else if rank == 3 {
        dsm.write_variable("w", 3, &world);

        println!("Rank 3: w = {}", dsm.local_variables[0].get_value());
    }

    world.barrier();

    //simulate some compare and exchange
    if rank == 0 {
        let success = dsm.compare_and_exchange("x", 43, 44, &world);
        println!("Success: {}", success);
    }
    else if rank == 1 {
        let success = dsm.compare_and_exchange("y", 1, 2, &world);
        println!("Success: {}", success);
    }
    else if rank == 2 {
        let success = dsm.compare_and_exchange("z", 2, 3, &world);
        println!("Success: {}", success);
    }
    else if rank == 3 {
        let success = dsm.compare_and_exchange("w", 3, 4, &world);
        println!("Success: {}", success);
    }




}