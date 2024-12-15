use std::io::{stdout, BufWriter, Write};

fn main() {
    let input: Vec<i64> = read_vec();
    let k2: i64 = input[0];
    let k3: i64 = input[1];
    let k5: i64 = input[2];
    let k6: i64 = input[3];

    let n: i64 = *vec![k2, k5, k6].iter().min().unwrap();

    let res: i64 = n * 256 + k3.min(k2 - n) * 32;

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
