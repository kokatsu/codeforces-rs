use std::io::{stdout, Write, BufWriter};

fn main() {
    let n: usize = read();
    let a: Vec<u64> = read_vec();

    let m: usize = 3;

    let mut counts: Vec<u64> = vec![0; m];
    for i in 0..n {
        counts[i%m] += a[i];
    }

    let max: u64 = *counts.iter().max().unwrap();

    let res: &str =
        if counts[0] == max {
            "chest"
        }
        else if counts[1] == max {
            "biceps"
        }
        else {
            "back"
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