use std::io::{stdout, BufWriter, Write};

fn main() {
    let n: usize = read();

    let mut x: Vec<i64> = vec![0; n];
    let mut h: Vec<i64> = vec![0; n];
    for i in 0..n {
        let input: Vec<i64> = read_vec();
        x[i] = input[0];
        h[i] = input[1];
    }

    let res: usize = (0..n).fold(0, |res, i| {
        if i == 0 || i == n - 1 || x[i] - h[i] > x[i - 1] {
            res + 1
        } else if x[i] + h[i] < x[i + 1] {
            x[i] += h[i];
            res + 1
        } else {
            res
        }
    });

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
