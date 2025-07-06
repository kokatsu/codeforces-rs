use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let _n: usize = read();
    let a: Vec<u64> = read_vec();

    let g: u64 = a.iter().fold(0, |g, &x| gcd(x, g));
    let s: u64 = (g as f64).sqrt().floor() as u64;

    let mut res: u64 = 0;
    for i in 1..=s {
        if g % i == 0 {
            res += 1;
            if i != g / i {
                res += 1;
            }
        }
    }

    writeln!(out, "{}", res).unwrap();
}

#[allow(dead_code)]
fn gcd(mut x: u64, mut y: u64) -> u64 {
    while y != 0 {
        (x, y) = (y, x % y);
    }
    x
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
