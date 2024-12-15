use std::io::{stdout, BufWriter, Write};

fn digit_sum(mut x: u64) -> u64 {
    let mut ret: u64 = 0;
    while x > 0 {
        ret += x % 10;
        x /= 10;
    }
    ret
}

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let n: u64 = read();

        let mut res: u64 = n;
        while gcd(res, digit_sum(res)) == 1 {
            res += 1;
        }

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
