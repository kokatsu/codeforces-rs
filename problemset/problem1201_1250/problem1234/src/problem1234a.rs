use std::io::{stdout, Write, BufWriter};

fn main() {
    let q: usize = read();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..q {
        let n: u64 = read();
        let a: Vec<u64> = read_vec();

        let x: u64 = *a.iter().min().unwrap();
        let y: u64 = (a.iter().sum::<u64>() + n - 1) / n;

        let res: u64 = x.max(y);

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