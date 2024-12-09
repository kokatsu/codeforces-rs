use std::io::{stdout, BufWriter, Write};

fn main() {
    let n: usize = read();
    let mut a: Vec<u64> = read_vec();

    a.sort();

    let is_ok: bool = (0..n - 2).fold(false, |is_ok, i| {
        if a[i] + a[i + 1] > a[i + 2] {
            true
        } else {
            is_ok
        }
    });

    let res: &str = if is_ok { "YES" } else { "NO" };

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
