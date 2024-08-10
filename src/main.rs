use rand::thread_rng;
use rand_distr::{Binomial, Distribution};
use rayon::prelude::*;
use std::time::Instant;

const ROLL_COUNT: u64 = 231;
const PROBABILITY_OF_ONE: f64 = 0.25;

fn find_ones(attempt_count: u64) -> u64 {
    let binomial = Binomial::new(ROLL_COUNT, PROBABILITY_OF_ONE).unwrap();

    let max_ones = (0..attempt_count)
        .into_par_iter()
        .map(|_| binomial.sample(&mut thread_rng()))
        .max()
        .unwrap_or(0);

    max_ones
}

fn main() {
    let start = Instant::now();
    let result = find_ones(1_000_000_000);
    let duration = start.elapsed();
    println!("Number of ones: {}", result);
    println!(
        "Time elapsed in seconds for find_ones() is: {:?}",
        duration.as_secs()
    );
}
