use std::io::{stdout, Write, BufWriter};
use std::collections::HashSet;

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let n: usize = read();

        let set: HashSet<String> =
            (0..n)
            .fold(HashSet::new(), |mut set, _| {
                let s: String = read_string();
                if s.contains("1") {
                    set.insert(s);
                }
                set
            });

        let res: &str = if set.len() == 1 { "SQUARE" } else { "TRIANGLE" };

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