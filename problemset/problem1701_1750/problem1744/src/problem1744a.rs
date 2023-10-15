use std::io::{stdout, Write, BufWriter};
use std::collections::HashSet;

fn main() {
    let t: usize = read();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let _n: usize = read();
        let a: Vec<usize> = read_vec();
        let s: Vec<char> = read_string().chars().collect();

        let mut set: HashSet<usize> = HashSet::new();
        let res: &str =
            a
            .iter()
            .fold("YES", |res, &x| {
                if set.contains(&x) {
                    res
                }
                else {
                    set.insert(x);
                    let pos: Vec<usize> =
                        a
                        .iter()
                        .enumerate()
                        .filter(|(_, &b)| b == x)
                        .map(|(j, _)| j)
                        .collect();
                    if pos.iter().all(|&p| s[p] == s[pos[0]]) {
                        res
                    }
                    else {
                        "NO"
                    }
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