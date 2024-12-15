use std::collections::HashMap;
use std::io::{stdout, BufWriter, Write};

fn main() {
    let n: u64 = read();
    let a: Vec<u64> = read_vec();

    let map: HashMap<u64, u64> = a
        .iter()
        .enumerate()
        .fold(HashMap::new(), |mut map, (i, x)| {
            *map.entry(*x).or_insert(0) = i as u64;
            map
        });

    let _m: u64 = read();
    let b: Vec<u64> = read_vec();

    let (l, r): (u64, u64) = b.iter().fold((0, 0), |(l, r), x| {
        let y: u64 = *map.get(&x).unwrap();
        (l + y + 1, r + n - y)
    });

    let mut out = BufWriter::new(stdout().lock());
    writeln!(out, "{} {}", l, r).unwrap();
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
