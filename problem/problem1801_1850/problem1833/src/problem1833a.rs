use std::io::{stdout, Write, BufWriter};
use std::collections::HashSet;

fn main() {
    let t: usize = read();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let n: usize = read();
        let s: Vec<char> = read_string().chars().collect();

        let set: HashSet<String> =
            (0..n-1)
            .fold(HashSet::new(), |mut set, i| {
                set.insert(s[i..i+2].iter().collect::<String>());
                set
            });

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