use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let (a, b): (u64, u64) = {
            let input: Vec<u64> = read_vec();
            (input[0], input[1])
        };

        let (n, m): (u64, u64) = {
            let input: Vec<u64> = read_vec();
            (input[0], input[1])
        };

        let d: u64 = n / (m + 1);
        let x: u64 = (a * m).min(b * (m + 1)) * d;

        let r: u64 = n % (m + 1);
        let y: u64 = a.min(b) * r;

        let res: u64 = x + y;

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
