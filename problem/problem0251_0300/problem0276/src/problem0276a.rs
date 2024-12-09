use std::io::{stdout, BufWriter, Write};

fn main() {
    let input: Vec<i64> = read_vec();
    let n: i64 = input[0];
    let k: i64 = input[1];

    let res: i64 = (0..n).fold(i64::MIN, |res, _| {
        let input: Vec<i64> = read_vec();
        let f: i64 = input[0];
        let t: i64 = input[1];
        if t <= k {
            res.max(f)
        } else {
            res.max(f - t + k)
        }
    });

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
