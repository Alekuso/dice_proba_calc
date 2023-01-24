use rand::prelude::*;
use rayon::prelude::*;
use std::io;
use std::io::Write;
use std::time::Instant;

#[macro_use]
mod scan;

fn main() {
    print!("Number of throws: ");
    io::stdout().flush().unwrap(); // For some reason the print! macro doesnt flush and wont print anything.

    let dices = scan!(u128);
    println!();

    let now = Instant::now();

    let cpus = rayon::current_num_threads();
    let result: usize = (0..cpus)
        .into_par_iter()
        .map(|_| {
            let mut rng = thread_rng();
            let mut on_one = 0;
            for _ in 0..(dices / cpus as u128) {
                let roll = rng.gen_range(1..=6);

                if roll == 1 {
                    on_one += 1;
                }
            }
            on_one
        })
        .sum();

    let time = now.elapsed();
    println!(
        "P(\"1\")={0}\nGot: {1}\nTime taken: {2}",
        result as f64 / dices as f64,
        format!("{}/{}", result, dices),
        if time.as_millis() >= 1000 {
            format!("{:.2}s", time.as_millis() as f64 / 1000.0)
        } else {
            format!("{:.2}ms", time.as_micros() as f64 / 1000.0)
        }
    );
}
