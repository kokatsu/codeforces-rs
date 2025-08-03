use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let n: usize = read();
        let a: Vec<u32> = read_vec();

        let mut ok: bool = true;
        for i in 1..n - 1 {
            let l: u32 = lcm(a[i - 1], a[i]);
            let r: u32 = lcm(a[i], a[i + 1]);
            if gcd(l, r) != a[i] {
                ok = false;
                break;
            }
        }

        let res: &str = if ok { "YES" } else { "NO" };

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
fn lcm(x: u32, y: u32) -> u32 {
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
