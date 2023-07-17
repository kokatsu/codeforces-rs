use std::io::{stdout, Write, BufWriter};

fn main() {
    let t: usize = read();

    let a: Vec<u64> =
        (0..=10u64.pow(5))
        .map(|i| i * (i + 1) / 2)
        .collect();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let input: Vec<u64> = read_vec();
        let n: u64 = input[0];
        let k: u64 = input[1];

        let p: u64 = a.partition_point(|&x| x < k) as u64;

        let res: String =
            (0..n)
            .fold(String::new(), |res, i| {
                if i == n - p - 1 || i == n + a[p as usize] - p - k {
                    res + "b"
                }
                else {
                    res + "a"
                }
            });

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