use std::io::{stdout, Write, BufWriter};

const MOD: u64 = 998_244_353;

fn factorial_mod(x: u64) -> u64 {
    (2..=x).fold(1, |res, i| (res * i) % MOD)
}

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let n: u64 = read();

        let res: u64 =
            if n % 2 == 0 {
                factorial_mod(n/2).pow(2) % MOD
            }
            else {
                0
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