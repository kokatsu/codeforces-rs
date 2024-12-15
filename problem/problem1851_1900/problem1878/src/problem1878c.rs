use std::io::{stdout, Write, BufWriter};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let input: Vec<u64> = read_vec();
        let (n, k, x): (u64, u64, u64) = (input[0], input[1], input[2]);

        let low: u64 = triangular_number(k);
        let high: u64 = triangular_number(n) - triangular_number(n-k);

        let res: &str = if low <= x && x <= high { "YES" } else { "NO" };

        writeln!(out, "{}", res).unwrap();
    }
}

fn triangular_number(x: u64) -> u64 {
    x * (x + 1) / 2
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