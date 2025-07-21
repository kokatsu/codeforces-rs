use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let n: usize = read();
    let mut t: Vec<char> = read_string().chars().collect();

    let mut divisors: Vec<usize> = Vec::new();
    for i in 1..=n {
        if n % i == 0 {
            divisors.push(i);
        }
    }

    for &d in divisors.iter() {
        t[0..d].reverse();
    }

    let res: String = t.iter().collect();

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
