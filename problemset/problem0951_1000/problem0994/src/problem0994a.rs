use std::collections::HashSet;
use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let (_n, _m): (usize, usize) = {
        let input: Vec<usize> = read_vec();
        (input[0], input[1])
    };

    let x: Vec<u8> = read_vec();
    let y: Vec<u8> = read_vec();

    let s: HashSet<u8> = y.into_iter().collect();

    let v: Vec<u8> = x.into_iter().filter(|a| s.contains(a)).collect();

    let res: String = v
        .iter()
        .map(|&x| x.to_string())
        .collect::<Vec<String>>()
        .join(" ");

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
