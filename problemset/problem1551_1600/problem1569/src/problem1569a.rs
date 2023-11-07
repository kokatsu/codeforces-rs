use std::io::{stdout, Write, BufWriter};

fn main() {
    let t: usize = read();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let n: usize = read();
        let s: Vec<char> = read_string().chars().collect();

        let (l, r): (i64, i64) =
            (0..n-1)
            .fold((-1, -1), |(l, r), i| {
                if s[i] != s[i+1] {
                    let j: i64 = i as i64;
                    (j+1, j+2)
                }
                else {
                    (l, r)
                }
            });

        writeln!(out, "{} {}", l, r).unwrap();
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