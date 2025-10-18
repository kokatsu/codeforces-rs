use std::collections::HashSet;
use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let _n: u8 = read();
    let a: Vec<u8> = read_vec();

    let set: HashSet<u8> = a.into_iter().collect();

    let mut boring: u8 = 0;
    let mut res: u8 = 0;
    for i in 1..=90 {
        res += 1;
        if set.contains(&i) {
            boring = 0;
        } else {
            boring += 1;
            if boring >= 15 {
                break;
            }
        }
    }

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
