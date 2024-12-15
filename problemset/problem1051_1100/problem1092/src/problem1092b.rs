use std::io::{stdout, BufWriter, Write};

fn main() {
    let n: usize = read();
    let mut a: Vec<u64> = read_vec();

    a.sort();

    let res: u64 = (0..n / 2).fold(0, |res, i| res + a[i * 2 + 1] - a[i * 2]);

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
