use std::io::{stdout, Write, BufWriter};

fn remove_zeros(x: u64) -> u64 {
    let mut base: u64 = 1;
    let mut rem: u64 = x;
    let mut ret: u64 = 0;
    while rem > 0 {
        if rem % 10 > 0 {
            ret += rem % 10 * base;
            base *= 10;
        }
        rem /= 10;
    }
    ret
}

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let a: u64 = read();
    let b: u64 = read();

    let x: u64 = remove_zeros(a);
    let y: u64 = remove_zeros(b);
    let z: u64 = remove_zeros(a+b);

    let res: &str = if x + y == z { "YES" } else { "NO" };

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