use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let n: usize = read();

    const LIMIT: usize = 10usize.pow(7);
    let mut is_prime: Vec<bool> = vec![true; LIMIT + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 2..=LIMIT {
        if is_prime[i] {
            let mut j: usize = i * 2;
            while j <= LIMIT {
                is_prime[j] = false;
                j += i;
            }
        }
    }
    let primes: Vec<usize> = (2..=LIMIT).filter(|&x| is_prime[x]).collect();

    let res: String = primes
        .iter()
        .take(n)
        .map(|p| p.to_string())
        .collect::<Vec<String>>()
        .join(" ");

    writeln!(out, "{}", res).unwrap();
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
