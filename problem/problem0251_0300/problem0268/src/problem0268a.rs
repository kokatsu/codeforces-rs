use std::io::{stdout, Write, BufWriter};

const UPPER_LIMIT: usize = 101;

fn main() {
    let n: usize = read();

    let mut home: Vec<i64> = vec![0; UPPER_LIMIT];
    let mut guest: Vec<i64> = vec![0; UPPER_LIMIT];
    for _ in 0..n {
        let input: Vec<usize> = read_vec();
        let h: usize = input[0];
        let a: usize = input[1];

        home[h] += 1;
        guest[a] += 1;
    }

    let res: i64 = (0..UPPER_LIMIT).fold(0, |res, i| res + home[i] * guest[i]);

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