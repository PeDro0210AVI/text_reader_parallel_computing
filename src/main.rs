use std::{collections::HashMap, env, fs::File, io::Read};
mod in_sequence;
mod parallel;

fn main() {
    let num_thread = *&std::env::args().collect::<Vec<String>>()[0]
        .parse::<usize>()
        .unwrap_or(4);

    // in sequence

    let sequence_instant = std::time::Instant::now();
    in_sequence::in_sequence();
    println!("");
    let sequence_duration = sequence_instant.elapsed();

    // parallel
    let parallel_instant = std::time::Instant::now();
    parallel::parallel(num_thread);
    println!("");
    let parallel_duration = parallel_instant.elapsed();

    print!(
        "Sequence: {:?}\nParallel: {:?}\n",
        sequence_duration, parallel_duration
    );
}
