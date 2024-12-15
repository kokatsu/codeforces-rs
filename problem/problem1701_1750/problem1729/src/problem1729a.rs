use std::io::{stdout, Write, BufWriter};
use std::cmp::Ordering;

fn main() {
    let t: usize = read();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let input: Vec<i64> = read_vec();
        let a: i64 = input[0];
        let b: i64 = input[1];
        let c: i64 = input[2];

        let x: i64 = a - 1;
        let y: i64 = (b-c).abs() + c - 1;

        let res: i64 = match x.cmp(&y) {
            Ordering::Less => 1,
            Ordering::Greater => 2,
            Ordering::Equal => 3,
        };

        writeln!(out, "{}", res).unwrap();
    }
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