use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let _n: usize = read();
        let a: Vec<u64> = read_vec();

        let p: u64 = a.iter().step_by(2).fold(a[0], |p, &x| gcd(p, x));
        let q: u64 = a.iter().skip(1).step_by(2).fold(a[1], |q, &x| gcd(q, x));

        let res: u64 = if a.iter().skip(1).step_by(2).all(|&x| x % p != 0) {
            p
        } else if a.iter().step_by(2).all(|&x| x % q != 0) {
            q
        } else {
            0
        };

        writeln!(out, "{}", res).unwrap();
    }
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
