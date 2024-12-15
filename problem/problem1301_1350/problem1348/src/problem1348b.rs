use std::io::{stdout, Write, BufWriter};
use std::collections::HashSet;

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let input: Vec<usize> = read_vec();
        let (n, k): (usize, usize) = (input[0], input[1]);

        let a: HashSet<u32> = read_vec::<u32>().iter().cloned().collect();

        let l: usize = a.len();
        if l > k {
            writeln!(out, "-1").unwrap();
            continue;
        }

        let mut s: String = a
            .iter()
            .fold(String::new(), |s, x| {
                if s.is_empty() {
                    x.to_string()
                }
                else {
                    s + " " + &x.to_string()
                }
            });

        for _ in l..k {
            s += " 1"
        }

        let res: String = (0..n)
            .fold((n*k).to_string() + "\n", |res, i| {
                if i == 0 {
                    res + &s.clone()
                }
                else {
                    res + " " + &s.clone()
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