use std::io::{stdout, BufWriter, Write};

fn main() {
    let input: Vec<i64> = read_vec();
    let (x, y): (i64, i64) = (input[0], input[1]);

    let n: i64 = read();

    let m: i64 = 10_i64.pow(9) + 7;

    let res: i64 = match n % 6 {
        1 => (x + m) % m,
        2 => (y + m) % m,
        3 => (y - x + m * 2) % m,
        4 => (m - x) % m,
        5 => (m - y) % m,
        _ => (x - y + m * 2) % m,
    };

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
