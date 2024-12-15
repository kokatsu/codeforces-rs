use std::io::{stdout, Write, BufWriter};

fn main() {
    let t: usize = read();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let input: Vec<usize> = read_vec();
        let (n, k): (usize, usize) = (input[0], input[1]);

        let letters: String = read_string();

        let mut cum: Vec<u64> = vec![0; n+1];
        for (i, c) in letters.chars().enumerate() {
            cum[i+1] += cum[i];
            if c == 'W' {
                cum[i+1] += 1;
            }
        }

        let res: u64 = (k..=n)
            .fold(u64::MAX, |res, i| res.min(cum[i]-cum[i-k]));

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