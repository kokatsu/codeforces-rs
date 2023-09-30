use std::io::{stdout, Write, BufWriter};

fn main() {
    let t: usize = read();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let n: usize = read();
        let a: Vec<u64> = read_vec();

        let max: u64 = *a.iter().max().unwrap();

        let indexes: Vec<usize> =
            a
            .iter()
            .enumerate()
            .filter(|(_1, &x)| x == max)
            .map(|(i, _)| i)
            .collect();

        let res: i64 =
            indexes
            .iter()
            .fold(-1, |res, &i| {
                if (i > 0 && a[i-1] < max) || (i < n - 1 && a[i+1] < max) {
                    i as i64 + 1
                }
                else {
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