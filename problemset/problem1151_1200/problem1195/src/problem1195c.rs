use std::io::{stdout, Write, BufWriter};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let n: usize = read();
    let h1: Vec<u64> = read_vec();
    let h2: Vec<u64> = read_vec();

    let (a, b, c): (u64, u64, u64) = (0..n)
        .fold((0, 0, 0), |(a, b, c), i| {
            (b.max(c), a.max(c) + h1[i], a.max(b) + h2[i])
        });

    let res: u64 = a.max(b).max(c);

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