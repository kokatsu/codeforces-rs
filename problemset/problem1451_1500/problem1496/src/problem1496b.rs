use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let (n, k): (usize, u64) = {
            let input: Vec<u64> = read_vec();
            (input[0] as usize, input[1])
        };

        let mut a: Vec<u64> = read_vec();

        a.sort();

        let m: u64 = n as u64;

        let mex: u64 = (0..n).find(|&i| a[i] != i as u64).unwrap_or(n) as u64;
        let max: u64 = a[n - 1];

        let res: u64 = if k == 0 {
            m
        } else if max < mex {
            m + k
        } else if a.iter().any(|&x| x == (max + mex).div_ceil(2)) {
            m
        } else {
            m + 1
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
