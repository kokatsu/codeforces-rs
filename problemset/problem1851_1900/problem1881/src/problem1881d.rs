use std::collections::HashMap;
use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let n: usize = read();
        let a: Vec<u32> = read_vec();

        let mut primes: HashMap<u32, usize> = HashMap::new();

        for x in a {
            let mut num = x;
            for i in 2..=((num as f64).sqrt() as u32) {
                while num % i == 0 {
                    *primes.entry(i).or_insert(0) += 1;
                    num /= i;
                }
            }
            if num > 1 {
                *primes.entry(num).or_insert(0) += 1;
            }
        }

        let res: &str = if primes.values().all(|&p| p % n == 0) {
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
