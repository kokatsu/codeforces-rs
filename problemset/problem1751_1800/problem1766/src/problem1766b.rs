use std::collections::HashSet;
use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let n: usize = read();
        let s: Vec<char> = read_string().chars().collect();

        let mut set: HashSet<String> = HashSet::new();

        let res: String = (3..n).fold("NO".to_string(), |res, i| {
            let x: String = s[i - 3..=i - 2].iter().collect();
            set.insert(x);
            let y: String = s[i - 1..=i].iter().collect();
            if set.contains(&y) {
                "YES".to_string()
            } else {
                res
            }
        });

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
