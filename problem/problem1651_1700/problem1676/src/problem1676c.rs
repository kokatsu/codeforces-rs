use std::io::{stdout, Write, BufWriter};

fn calculate_difference(x: &str, y: &str) -> u32 {
    x.as_bytes().iter().zip(y.as_bytes().iter()).fold(0, |num, (u, v)| num + u.abs_diff(*v) as u32)
}

fn main() {
    let t: usize = read();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let input: Vec<usize> = read_vec();
        let n: usize = input[0];
        let _m: usize = input[1];

        let s: Vec<String> =
            (0..n)
            .map(|_| read_string())
            .collect();

        let res: u32 =
            (0..n)
            .flat_map(|i| (i+1..n).map(move |j| (i, j)))
            .fold(std::u32::MAX, |res, (i, j)| res.min(calculate_difference(&s[i], &s[j])));

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