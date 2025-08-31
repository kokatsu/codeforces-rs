use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let n: usize = read();
    let s: Vec<u32> = read_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    let is_lucky = |s: &[u32]| s.iter().all(|&c| c == 4 || c == 7);
    let same_digit = |s: &[u32]| {
        let (left, right) = s.split_at(n / 2);
        left.iter().sum::<u32>() == right.iter().sum::<u32>()
    };

    let res = if is_lucky(&s) && same_digit(&s) {
        "YES"
    } else {
        "NO"
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
