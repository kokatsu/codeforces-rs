use std::io::{stdout, Write, BufWriter};

fn main() {
    let t: usize = read();

    let upper: usize = 10_usize.pow(6) + 1;

    let mut sieve: Vec<bool> = vec![true; upper];
    sieve[0] = false;
    sieve[1] = false;
    let mut d: usize = 2;
    while d * d < upper {
        if sieve[d] {
            for i in (d*d..upper).step_by(d) {
                sieve[i] = false;
            }
        }
        d += 1;
    }

    let primes: Vec<u64> =
        (0..upper)
        .filter(|&i| sieve[i])
        .map(|i| i as u64)
        .collect();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let d: u64 = read();

        let i: usize = primes.partition_point(|&p| p < d + 1);
        let j: usize = primes.partition_point(|&p| p < d + primes[i]);

        let res: u64 = primes[i] * primes[j];

        writeln!(out, "{}", res).unwrap();
    }
}

#[allow(dead_code)]
fn read_string() -> String {
    let mut s: String = String::new();
    std::io::stdin().read_line(&mut s).ok();

    s.trim().to_string()
}

#[allow(dead_code)]
fn read<T: std::str::FromStr>() -> T {
    read_string().parse().ok().unwrap()
}

#[allow(dead_code)]
fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    read_string()
        .split_whitespace()
        .map(|v| v.parse().ok().unwrap())
        .collect()
}