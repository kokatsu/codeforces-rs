use std::collections::HashSet;
use std::io::{stdout, BufWriter, Write};

fn main() {
    let t: usize = read();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let n: i64 = read();
        let a: Vec<i64> = read_vec();

        let mut set: HashSet<i64> = HashSet::new();

        let res: i64 = a
            .iter()
            .rev()
            .fold((n, true), |(res, is_ok), x| {
                if !is_ok || set.contains(&x) {
                    (res, false)
                } else {
                    set.insert(*x);
                    (res - 1, is_ok)
                }
            })
            .0;

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
