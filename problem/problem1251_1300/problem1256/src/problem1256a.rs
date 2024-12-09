use std::io::{stdout, Write, BufWriter};

fn main() {
    let q: usize = read();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..q {
        let input: Vec<i64> = read_vec();
        let a: i64 = input[0];
        let b: i64 = input[1];
        let n: i64 = input[2];
        let s: i64 = input[3];

        let m: i64 = a.min(s/n);

        let res: &str =
            if s - n * m <= b {
                "YES"
            }
            else {
                "NO"
            };

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