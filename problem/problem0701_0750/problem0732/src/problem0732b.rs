use std::io::{stdout, Write, BufWriter};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let input: Vec<usize> = read_vec();
    let (n, k): (usize, i64) = (input[0], input[1] as i64);

    let mut a: Vec<i64> = read_vec();

    let mut number: i64 = 0;
    for i in 1..n {
        let walks: i64 = a[i-1] + a[i];
        if walks < k {
            let diff: i64 = k - walks;
            number += diff;
            a[i] += diff;
        }
    }

    let res: String = (1..n)
        .fold(number.to_string() + "\n" + &a[0].to_string(), |res, i| res + " " + &a[i].to_string());

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