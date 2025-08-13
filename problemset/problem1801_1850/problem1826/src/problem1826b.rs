use std::io::{stdout, BufWriter, Write};

fn is_palindrome_array<T: PartialEq>(a: &[T]) -> bool {
    let n = a.len();
    for i in 0..n / 2 {
        if a[i] != a[n - 1 - i] {
            return false;
        }
    }
    true
}

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let n: usize = read();
        let a: Vec<u32> = read_vec();

        let res: u32 = if is_palindrome_array(&a) {
            0
        } else {
            let b: Vec<u32> = (0..n / 2).map(|i| a[i].abs_diff(a[n - 1 - i])).collect();
            let g: u32 = b.iter().fold(0, |acc, &x| gcd(acc, x));
            g
        };

        writeln!(out, "{}", res).unwrap();
    }
}

#[allow(dead_code)]
fn gcd(mut x: u32, mut y: u32) -> u32 {
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
