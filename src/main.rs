use clap::Parser;
use statrs::distribution::{ContinuousCDF, StudentsT};
use std::process;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    /// correlation coefficient r
    #[arg(long, allow_hyphen_values = true)]
    r: f64,

    /// sample size n
    #[arg(long)]
    n: usize,
}

fn main() {
    let args = Args::parse();
    let r = args.r;
    let n = args.n as f64;

    // 異常値チェック
    if r < -1.0 || r > 1.0 {
        eprintln!("Error: correlation coefficient r must be between -1 and 1 (got {})", r);
        process::exit(1);
    }
    if n < 3.0 {
        eprintln!("Error: sample size n must be at least 3 (got {})", n);
        process::exit(1);
    }

    let df = n - 2.0;
    let t  = r * ((df) / (1.0 - r * r)).sqrt();

    // two-sided p-value
    let p = {
        let dist = StudentsT::new(0.0, 1.0, df).unwrap();
        2.0 * (1.0 - dist.cdf(t.abs()))
    };

    println!("t={}, p={}", t, p);
}
