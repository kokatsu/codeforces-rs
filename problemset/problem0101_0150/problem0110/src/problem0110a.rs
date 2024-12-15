use std::io::{stdout, BufWriter, Write};

fn is_lucky_digit(x: i64) -> bool {
    x == 4 || x == 7
}

fn main() {
    let n: String = read_string();

    let l: i64 = n
        .chars()
        .filter(|&x| is_lucky_digit(x.to_string().parse::<i64>().unwrap()))
        .collect::<Vec<_>>()
        .len() as i64;

    let res: &str = if is_lucky_digit(l) { "YES" } else { "NO" };

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
