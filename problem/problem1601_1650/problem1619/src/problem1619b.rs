use std::collections::HashSet;
use std::io::{stdout, BufWriter, Write};

fn main() {
    let t: usize = read();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let n: i64 = read();

        let mut set: HashSet<i64> = HashSet::new();

        let s: i64 = (n as f64).sqrt().floor() as i64;
        for i in 1..=s {
            set.insert(i * i);
        }

        let c: i64 = (n as f64).cbrt().floor() as i64;
        for i in 1..=c {
            set.insert(i * i * i);
        }

        let res: usize = set.len();

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
