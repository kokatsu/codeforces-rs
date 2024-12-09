use std::io::{stdout, Write, BufWriter};

fn main() {
    let input: Vec<u64> = read_vec();
    let a: u64 = input[0];
    let b: u64 = input[1];
    let mut n: u64 = input[2];

    let mut cnt: u64 = 0;
    while n > 0 {
        let g: u64 =
            if cnt % 2 == 0 {
                gcd(n, a)
            }
            else {
                gcd(n, b)
            };

        n -= g;
        cnt += 1;
    }

    let res: u64 = (cnt + 1) % 2;

    let mut out = BufWriter::new(stdout().lock());
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