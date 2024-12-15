use std::io::{stdout, Write, BufWriter};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let (a, b): (u64, u64) = {
            let input: Vec<u64> = read_vec();
            (input[0], input[1])
        };

        let lcm: u64 = a / gcd(a, b) * b;

        let res: u64 =
            if lcm == b {
                b / a * lcm
            }
            else {
                lcm
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