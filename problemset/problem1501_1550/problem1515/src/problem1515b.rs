use std::io::{stdout, BufWriter, Write};

fn is_square(x: u64) -> bool {
    let r: u64 = (x as f64).sqrt().floor() as u64;
    r * r == x
}

fn can_create(x: u64, y: u64) -> bool {
    x % y == 0 && is_square(x / y)
}

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let n: u64 = read();

        let res: &str = if can_create(n, 2) || can_create(n, 4) {
            "YES"
        } else {
            "NO"
        };

        writeln!(out, "{}", res).unwrap();
    }
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
