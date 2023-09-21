use std::io::{stdout, Write, BufWriter};

fn main() {
    let t: usize = read();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let k: u64 = read();

        let s: u64 = (k as f64).sqrt().floor() as u64;

        let (r, c): (u64, u64) =
            if s * s == k {
                (s, 1)
            }
            else {
                let m: u64 = k - s * s;
                let n: u64 = s + 1;
                if m <= s {
                    (m, n)
                }
                else {
                    (n, n*2-m)
                }
            };

        writeln!(out, "{} {}", r, c).unwrap();
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