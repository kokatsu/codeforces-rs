use std::io::{stdout, BufWriter, Write};

fn main() {
    let n: usize = read();
    let a: Vec<usize> = read_vec();

    let num: usize = a
        .iter()
        .fold((0, 0, 0), |(num, u, v), x| {
            if *x == 25 {
                (num + 1, u + 1, v)
            } else if *x == 50 {
                if u > 0 {
                    (num + 1, u - 1, v + 1)
                } else {
                    (num, u, v)
                }
            } else {
                if u > 0 && v > 0 {
                    (num + 1, u - 1, v - 1)
                } else if u > 2 {
                    (num + 1, u - 3, v)
                } else {
                    (num, u, v)
                }
            }
        })
        .0;

    let res: &str = if num == n { "YES" } else { "NO" };

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
