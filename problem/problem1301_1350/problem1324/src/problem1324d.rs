use std::io::{stdout, BufWriter, Write};

fn main() {
    let n: usize = read();
    let a: Vec<i64> = read_vec();
    let b: Vec<i64> = read_vec();

    let mut x: Vec<i64> = a
        .into_iter()
        .zip(b.into_iter())
        .map(|(u, v)| u - v)
        .collect();

    x.sort();

    let res: i64 = (0..n - 1).fold(0, |res, i| {
        res + (n - x[i + 1..n].partition_point(|&v| v <= -x[i]) - i - 1) as i64
    });

    let mut out = BufWriter::new(stdout().lock());
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
