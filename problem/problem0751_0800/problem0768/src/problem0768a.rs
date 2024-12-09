use std::io::{stdout, BufWriter, Write};

fn main() {
    let n: usize = read();
    let a: Vec<u64> = read_vec();

    let (min, max): (u64, u64) = a
        .iter()
        .fold((u64::MAX, 0), |(min, max), &x| (min.min(x), max.max(x)));

    let min_count: usize = a.iter().filter(|&&x| x == min).count();
    let max_count: usize = a.iter().filter(|&&x| x == max).count();

    let res: usize = if min == max {
        0
    } else {
        n - min_count - max_count
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
