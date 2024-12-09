use std::io::{stdout, Write, BufWriter};

fn main() {
    let input: Vec<i64> = read_vec();
    let n: i64 = input[0];
    let m: i64 = input[1];

    let res: &str =
        if n.min(m) % 2 == 1 {
            "Akshat"
        }
        else {
            "Malvika"
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