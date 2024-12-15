use std::io::{stdout, Write, BufWriter};

fn main() {
    let t: i64 = read();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let a: Vec<i64> = read_vec();

        writeln!(out, "{} {} {}", a[0], a[1], a[6]-a[0]-a[1]).unwrap();
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