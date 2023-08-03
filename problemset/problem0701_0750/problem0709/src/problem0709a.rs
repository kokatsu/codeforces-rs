use std::io::{stdout, Write, BufWriter};

fn main() {
    let input: Vec<u64> = read_vec();
    let _n: u64 = input[0];
    let b: u64 = input[1];
    let d: u64 = input[2];

    let a: Vec<u64> = read_vec();

    let res: u64 =
        a
        .iter()
        .filter(|x| x <= &&b)
        .fold((0, 0), |(res, section), &x| {
            if section + x <= d {
                (res, section+x)
            }
            else {
                (res+1, 0)
            }
        }).0;

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