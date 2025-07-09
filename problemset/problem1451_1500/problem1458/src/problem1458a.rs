use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let (n, _m): (usize, usize) = {
        let input: Vec<usize> = read_vec();
        (input[0], input[1])
    };

    let mut a: Vec<u64> = read_vec();
    let b: Vec<u64> = read_vec();

    a.sort();

    let g: u64 = (1..n).map(|i| a[i] - a[0]).fold(0, gcd);

    let res: String = b
        .iter()
        .map(|&x| gcd(g, a[0] + x).to_string())
        .collect::<Vec<String>>()
        .join(" ");

    writeln!(out, "{}", res).unwrap();
}

#[allow(dead_code)]
fn gcd(mut x: u64, mut y: u64) -> u64 {
    while y != 0 {
        (x, y) = (y, x % y);
    }
    x
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
