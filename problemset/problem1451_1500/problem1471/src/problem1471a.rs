use std::io::{stdout, Write, BufWriter};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let input: Vec<u64> = read_vec();
        let (_n, x): (u64, u64) = (input[0], input[1]);

        let a: Vec<u64> = read_vec();

        let min: u64 = a.iter().sum::<u64>().div_ceil(x);
        let max: u64 = a.iter().map(|&u| u.div_ceil(x)).sum();

        writeln!(out, "{} {}", min, max).unwrap();
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