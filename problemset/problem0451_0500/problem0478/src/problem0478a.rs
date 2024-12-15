use std::io::{stdout, BufWriter, Write};

fn main() {
    let c: Vec<i64> = read_vec();

    let sum: i64 = c.iter().sum();
    let (div, rem): (i64, i64) = (sum / 5, sum % 5);

    let res: i64 = if div > 0 && rem == 0 { div } else { -1 };

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
