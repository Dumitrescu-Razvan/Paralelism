mod dsm;

use mpi::traits::*;
use mpi::collective::SystemOperation;


fn main() {
    // Initialize the Universe
    let universe = mpi::initialize().unwrap();
    let world = universe.world();
    let rank = world.rank();
    let _size = world.size();

    // Create a DSM
    let mut dsm = dsm::DSM::new();

    // Add a variable
    dsm.add_variable("x".to_string(), 0);

    // Subscribe to the variable
    dsm.subscribe("x".to_string(), rank);

    // Notify the subscribers
    dsm.notify("x".to_string(), 42);

    // Finalize the Universe
    world.all_reduce_into(&rank, &mut 0, SystemOperation::sum());

}
