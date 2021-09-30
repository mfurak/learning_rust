use rand::Rng;
use std::{cmp::Ordering, ops::Div};

enum Comparator {
    High,
    Low,
    Correct,
}

fn main() {
    // setup experiment
    const BASE: u32 = 2;
    const RANGE_MAX: u32 = 101;
    const NUM_OF_EXP: u32 = 100_000_000;
    const STARTING_GUESS: u32 = RANGE_MAX / 2;

    // setup necessary variables
    let max_guesses: u32 = (RANGE_MAX as f32).log2().round() as u32;
    let mut tries_count: u32 = 0;

    for _i in 0..NUM_OF_EXP {
        let secret_number: u32 = rand::thread_rng().gen_range(1..RANGE_MAX + 1);
        // // sanity check -> result should be 1 no matter the NUM_OF_EXP
        // let secret_number: u32 = STARTING_GUESS;
        let mut guess: u32 = STARTING_GUESS;

        for j in 1..max_guesses {
            tries_count += 1;
            match try_guess(&guess, &secret_number) {
                Comparator::Correct => break,
                Comparator::High => guess -= STARTING_GUESS / BASE.pow(j),
                Comparator::Low => guess += STARTING_GUESS / BASE.pow(j),
            }
        }
    }
    println!("{}", (tries_count as f32).div(NUM_OF_EXP as f32))
}

fn try_guess(guess: &u32, secret_number: &u32) -> Comparator {
    match guess.cmp(secret_number) {
        Ordering::Less => Comparator::Low,
        Ordering::Greater => Comparator::High,
        Ordering::Equal => Comparator::Correct,
    }
}
