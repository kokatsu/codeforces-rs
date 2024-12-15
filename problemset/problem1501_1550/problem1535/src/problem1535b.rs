use std::io::{stdout, BufWriter, Write};

fn main() {
    let t: usize = read();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let _n: usize = read();
        let a: Vec<u64> = read_vec();

        let odds: Vec<u64> = a.iter().filter(|&x| x % 2 == 1).cloned().collect();
        let evens: Vec<u64> = a.iter().filter(|&x| x % 2 == 0).cloned().collect();

        let odd_count: usize = odds.len();
        let even_count: usize = evens.len();

        let mut res: usize = if even_count > 0 {
            even_count * (even_count - 1) / 2 + even_count * odd_count
        } else {
            0
        };
        for i in 0..odd_count {
            res += (i + 1..odd_count).fold(0, |num, j| {
                if gcd(odds[i], odds[j]) > 1 {
                    num + 1
                } else {
                    num
                }
            });
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
