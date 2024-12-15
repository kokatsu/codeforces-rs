use std::io::{stdout, BufWriter, Write};

fn main() {
    let input: Vec<usize> = read_vec();
    let _n: usize = input[0];
    let _m: usize = input[1];

    let mut a: Vec<i64> = read_vec();
    let b: Vec<i64> = read_vec();

    a.sort();

    let res: String = b
        .iter()
        .fold(String::new(), |res, &x| {
            res + " " + &a.partition_point(|&v| v <= x).to_string()
        })
        .trim()
        .to_string();

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
