use std::io::{stdout, Write, BufWriter};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let (n, x, y): (i64, i64, i64) = {
            let input: Vec<i64> = read_vec();
            (input[0], input[1], input[2])
        };

        let m: i64 = n / lcm(x, y);
        let (u, v): (i64, i64) = (n / x - m, n / y - m);
        let res: i64 = (n * 2 - u + 1) * u / 2 - (v + 1) * v / 2;

        writeln!(out, "{}", res).unwrap();
    }
}

#[allow(dead_code)]
fn gcd(mut x: i64, mut y: i64) -> i64 {
    while y != 0 {
        (x, y) = (y, x % y);
    }
    x
}

#[allow(dead_code)]
fn lcm(x: i64, y: i64) -> i64 {
    x / gcd(x, y) * y
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