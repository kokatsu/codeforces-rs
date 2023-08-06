use std::io::{stdout, Write, BufWriter};
use std::collections::HashMap;

fn main() {
    let t: usize = read();

    let map: HashMap<char, i64> =
        "Timur"
        .chars()
        .fold(HashMap::new(), |mut map, x| {
            *map.entry(x).or_insert(0) += 1;
            map
        });

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let _n: usize = read();
        let s: String = read_string();

        let m: HashMap<char, i64> =
            s
            .chars()
            .fold(HashMap::new(), |mut map, x| {
                *map.entry(x).or_insert(0) += 1;
                map
            });

        let res: &str =
            if m == map {
                "YES"
            }
            else {
                "NO"
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