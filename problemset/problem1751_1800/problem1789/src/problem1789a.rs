use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let n: usize = read();
        let a: Vec<u32> = read_vec();

        let is_ok: bool = (0..n - 1).any(|i| (i + 1..n).any(|j| gcd(a[i], a[j]) <= 2));

        let res: &str = if is_ok { "Yes" } else { "No" };

        writeln!(out, "{}", res).unwrap();
    }
}

#[allow(dead_code)]
fn gcd(mut x: u32, mut y: u32) -> u32 {
    while y != 0 {
        (x, y) = (y, x % y);
    }
    x
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
