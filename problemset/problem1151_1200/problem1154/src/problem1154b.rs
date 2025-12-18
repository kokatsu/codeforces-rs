use std::collections::BTreeSet;
use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let _n: usize = read();
    let a: Vec<i8> = read_vec();

    let set: BTreeSet<i8> = a.into_iter().collect();
    let keys: Vec<i8> = set.into_iter().collect();

    let res: i8 = match keys.len() {
        1 => 0,
        2 => {
            let d: i8 = keys[1] - keys[0];
            if d % 2 == 0 {
                d / 2
            } else {
                d
            }
        }
        3 => {
            let d1: i8 = keys[1] - keys[0];
            let d2: i8 = keys[2] - keys[1];
            if d1 == d2 {
                d1
            } else {
                -1
            }
        }
        _ => -1,
    };

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
