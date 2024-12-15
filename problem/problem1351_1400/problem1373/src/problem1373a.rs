use std::io::{stdout, BufWriter, Write};

fn main() {
    let t: usize = read();

    let l: i64 = 10_i64.pow(9);

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let input: Vec<i64> = read_vec();
        let a: i64 = input[0];
        let b: i64 = input[1];
        let c: i64 = input[2];

        let x1: i64 = if a < c { 1 } else { -1 };

        let d: i64 = l / b;
        let x2: i64 = if c * d < a * b * d { b * d } else { -1 };

        writeln!(out, "{} {}", x1, x2).unwrap();
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
