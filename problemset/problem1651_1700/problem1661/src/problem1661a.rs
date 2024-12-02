use std::io::{stdout, Write, BufWriter};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let n: usize = read();
        let mut a: Vec<u64> = read_vec();
        let mut b: Vec<u64> = read_vec();

        if a[0] > b[0] {
            (a[0], b[0]) = (b[0], a[0]);
        }

        let res: u64 = (1..n)
            .fold(0, |res, i| {
                if a[i] > b[i] {
                    (a[i], b[i]) = (b[i], a[i])
                }
                res + a[i-1].abs_diff(a[i]) + b[i-1].abs_diff(b[i])
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