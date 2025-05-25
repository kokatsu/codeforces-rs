use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    let upper: usize = 10_usize.pow(6) + 1;

    let primes: Vec<u64> = {
        let mut sieve: Vec<bool> = vec![true; upper];
        sieve[0] = false;
        sieve[1] = false;
        let mut d: usize = 2;
        while d * d < upper {
            if sieve[d] {
                for i in (d * d..upper).step_by(d) {
                    sieve[i] = false;
                }
            }
            d += 1;
        }
        (0..upper).filter(|&i| sieve[i]).map(|i| i as u64).collect()
    };

    for _ in 0..t {
        let (x, k): (u64, u64) = {
            let input: Vec<u64> = read_vec();
            (input[0], input[1])
        };

        let is_prime: bool = if x < 2 {
            false
        } else {
            primes.iter().all(|&p| x == p || x % p != 0)
        };

        let res: &str = if (k == 1 && is_prime) || (x == 1 && k == 2) {
            "YES"
        } else {
            "NO"
        };

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
