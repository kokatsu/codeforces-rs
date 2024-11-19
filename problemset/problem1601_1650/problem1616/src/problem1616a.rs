use std::io::{stdout, Write, BufWriter};
use std::collections::HashMap;

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let _n: usize = read();
        let a: Vec<i8> = read_vec();

        let map: HashMap<i8, i8> = a
            .iter()
            .fold(HashMap::new(), |mut map, x| {
                *map.entry(x.abs()).or_insert(0) += 1;
                map
            });

        let vec: Vec<(i8, i8)> = map
            .into_iter()
            .collect::<Vec<_>>();

        let res: i8 = vec
            .iter()
            .map(|x| {
                if x.0 == 0 {
                    1
                }
                else {
                    x.1.min(2)
                }
            })
            .sum();

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