use std::io::{stdout, Write, BufWriter};

fn main() {
    let n: u64 = read();
    let a: Vec<u64> = read_vec();

    let s: u64 = a.iter().sum();

    let m: u64 =
        if n % s == 0 {
            s
        }
        else {
            n % s
        };

    let res: usize =
        a
        .iter()
        .enumerate()
        .fold((0, m), |(res, rem), (i, &x)| {
            if res > 0 {
                (res, rem)
            }
            else if rem <= x {
                (i+1, 0)
            }
            else {
                (res, rem-x)
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