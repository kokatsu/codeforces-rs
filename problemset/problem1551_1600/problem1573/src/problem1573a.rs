use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let _n: usize = read();
        let s: String = read_string();

        let v: Vec<u32> = s.chars().map(|c| c.to_digit(10).unwrap()).rev().collect();

        let res: u32 = v.iter().enumerate().fold(0, |res, (i, &x)| {
            if x > 0 {
                if i > 0 {
                    res + x + 1
                } else {
                    res + x
                }
            } else {
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
