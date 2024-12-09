use std::io::{stdout, Write, BufWriter};

fn main() {
    let n: usize = read();
    let x: Vec<i64> = read_vec();

    let mut out = BufWriter::new(stdout().lock());

    for i in 0..n {
        let (mn, mx): (i64, i64) =
            if i == 0 {
                (x[i+1]-x[i], x[n-1]-x[i])
            }
            else if i == n - 1 {
                (x[i]-x[i-1], x[i]-x[0])
            }
            else {
                ((x[i]-x[i-1]).min(x[i+1]-x[i]), (x[i]-x[0]).max(x[n-1]-x[i]))
            };

        writeln!(out, "{} {}", mn, mx).unwrap();
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