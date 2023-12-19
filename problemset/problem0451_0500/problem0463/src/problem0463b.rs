use std::io::{stdout, Write, BufWriter};

fn main() {
    let n: usize = read();
    let h: Vec<i64> = read_vec();

    let res: i64 =
        (0..n-1)
        .fold((-h[0], -h[0]), |(res, sum), i| (res.min(sum+h[i]-h[i+1]), sum+h[i]-h[i+1]))
        .0
        .abs();

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