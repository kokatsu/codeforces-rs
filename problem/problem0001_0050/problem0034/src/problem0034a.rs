use std::io::{stdout, BufWriter, Write};

fn main() {
    let n: usize = read();
    let a: Vec<u64> = read_vec();

    let index: usize = (0..n)
        .fold((0, u64::MAX), |(index, diff), i| {
            let d: u64 = a[i].abs_diff(a[(i + 1) % n]);
            if d < diff {
                (i, d)
            } else {
                (index, diff)
            }
        })
        .0;

    let mut out = BufWriter::new(stdout().lock());
    writeln!(out, "{} {}", index + 1, (index + 1) % n + 1).unwrap();
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
