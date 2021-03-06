use anyhow::Error;
use log::{info, log, trace, warn};
use rayon::prelude::*;

use regex::Regex;
use ssh_key::{rand_core::OsRng, Algorithm, LineEnding, PrivateKey};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::{fmt, thread};

use clap::Parser;

const COUNTER_THRESHOLD: usize = 10000;

#[derive(Parser)]
#[clap(name = "Sanity")]
#[clap(author = "Daniel Norred -  daniel@nor.red")]
#[clap(version = "1.0")]
#[clap(about = "Generates Vanity ED25519 SSH Keys", long_about = None)]
struct Cli {
    #[clap(long)]
    name: String,
    #[clap(long)]
    threads: u8,
}

#[derive(Debug)]
struct Counter {
    total: AtomicUsize,
    success: AtomicUsize,
}

impl Counter {
    /// Create new instance
    fn new() -> Self {
        Self {
            total: AtomicUsize::new(0),
            success: AtomicUsize::new(0),
        }
    }

    /// Count towards total numbers of fingerprints generated
    fn count_total(&self, accumulated_counts: usize) {
        self.total.fetch_add(accumulated_counts, Ordering::SeqCst);
    }

    /// Count towards total numbers of fingerprints matched
    fn count_success(&self) {
        self.success.fetch_add(1, Ordering::SeqCst);
    }

    /// Get number of total fingerprints generated
    fn get_total(&self) -> usize {
        self.total.load(Ordering::SeqCst)
    }

    /// Get number of total fingerprints matched
    fn get_success(&self) -> usize {
        self.success.load(Ordering::SeqCst)
    }
}

impl fmt::Display for Counter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} matched, {} total",
            self.get_success(),
            self.get_total(),
        )
    }
}

fn setup_summary(counter: Arc<Counter>) {
    let start = Instant::now();
    loop {
        thread::sleep(Duration::from_millis(1000));
        let secs_elapsed = start.elapsed().as_secs();
        println!(
            "Summary: {} (avg. {:.2} hash/s)",
            &counter,
            counter.get_total() as f64 / secs_elapsed as f64
        );
    }
}

fn main() -> Result<(), Error> {
    let cli = Cli::parse();
    println!("Started");
    let counter = Arc::new(Counter::new());
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads((cli.threads + 1).into())
        .build()
        .unwrap();

    for thread_id in 0..cli.threads {
        let reg = Regex::new(cli.name.as_str()).unwrap();
        let counter_cloned = Arc::clone(&counter);
        pool.spawn(move || {
            println!("Thread #{} Started", thread_id);
            let mut report_counter: usize = 0;
            loop {
                let unencrypted_key = PrivateKey::random(&mut OsRng, Algorithm::Ed25519).unwrap();
                let pubKey = unencrypted_key.public_key().to_openssh().unwrap();

                if reg.is_match(&pubKey) {
                    let priv_text = unencrypted_key.to_openssh(LineEnding::default()).unwrap();
                    counter_cloned.count_success();
                    println!("{}", pubKey);
                    println!("{}", counter_cloned)
                }
                report_counter += 1;
                if report_counter >= COUNTER_THRESHOLD {
                    counter_cloned.count_total(report_counter);
                    report_counter = 0;
                }
            }
        });
    }

    let counter_cloned = Arc::clone(&counter);

    pool.install(move || setup_summary(counter_cloned));

    Ok(())
}
