use std::io::{stdout, Write, BufWriter};

fn main() {
    let t: usize = read();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let _n: usize = read();
        let r: Vec<i64> = read_vec();

        let _m: usize = read();
        let b: Vec<i64> = read_vec();

        let u: i64 =
            r
            .iter()
            .fold((0, 0), |(max, sum), x| (max.max(sum+x), sum+x)).0;

        let v: i64 =
            b
            .iter()
            .fold((0, 0), |(max, sum), x| (max.max(sum+x), sum+x)).0;

        let res: i64 = u + v;

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