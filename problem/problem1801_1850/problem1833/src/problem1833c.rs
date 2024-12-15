use std::io::{stdout, Write, BufWriter};

fn main() {
    let t: usize = read();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let _n: usize = read();
        let a: Vec<u64> = read_vec();

        let (odd_min, even_min): (u64, u64) = a
            .iter()
            .fold((u64::MAX, u64::MAX), |(odd_min, even_min), &x| {
                if x % 2 == 1 {
                    (odd_min.min(x), even_min)
                }
                else {
                    (odd_min, even_min.min(x))
                }
            });

        let res: &str =
            if odd_min == u64::MAX || even_min == u64::MAX || odd_min < even_min {
                "YES"
            }
            else {
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