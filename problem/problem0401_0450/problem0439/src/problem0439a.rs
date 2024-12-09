use std::io::{stdout, Write, BufWriter};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let input: Vec<i64> = read_vec();
    let (n, d): (i64, i64) = (input[0], input[1]);

    let t: Vec<i64> = read_vec();

    let sum: i64 = t.iter().sum();
    let rem: i64 = d - sum - (n - 1) * 10;

    let res: i64 =
        if rem >= 0 {
            (n - 1) * 2 + rem / 5
        }
        else {
            -1
        };

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