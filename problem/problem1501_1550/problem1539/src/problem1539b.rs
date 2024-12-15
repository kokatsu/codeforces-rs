use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let (n, q): (usize, usize) = {
        let input: Vec<usize> = read_vec();
        (input[0], input[1])
    };

    let s: String = read_string();

    let a: Vec<u64> = {
        let mut t: Vec<u64> = vec![0; n + 1];
        for (i, c) in s.chars().enumerate() {
            let d: u64 = (c as u8 - 'a' as u8 + 1) as u64;
            t[i + 1] = t[i] + d;
        }
        t
    };

    for _ in 0..q {
        let (l, r): (usize, usize) = {
            let input: Vec<usize> = read_vec();
            (input[0] - 1, input[1])
        };

        let res: u64 = a[r] - a[l];

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
