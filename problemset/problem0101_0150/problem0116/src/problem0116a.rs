use std::io::{stdout, Write, BufWriter};

fn main() {
    let n: i64 = read();

    let mut cnt: i64 = 0;
    let mut res: i64 = 0;
    for _ in 0..n {
        let v: Vec<i64> = read_vec();

        cnt = cnt - v[0] + v[1];
        res = res.max(cnt);
    }

    let mut out = BufWriter::new(stdout().lock());
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