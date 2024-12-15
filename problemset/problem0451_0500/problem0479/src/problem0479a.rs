use std::io::{stdout, BufWriter, Write};

fn main() {
    let a: u64 = read();
    let b: u64 = read();
    let c: u64 = read();

    let u: u64 = a + b;
    let v: u64 = b + c;
    let x: u64 = a * b;
    let y: u64 = b * c;

    let res: u64 = *vec![u + c, u * c, a + v, a * v, x + c, x * c, a + y, a * y]
        .iter()
        .max()
        .unwrap();

    let mut out = BufWriter::new(stdout().lock());
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
