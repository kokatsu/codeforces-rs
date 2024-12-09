use std::io::{stdout, Write, BufWriter};

fn main() {
    let mut n: usize = read();

    let people: Vec<&str> = vec!["Sheldon", "Leonard", "Penny", "Rajesh", "Howard"];

    let mut d: usize = 1;
    while d * 5 < n {
        n -= d * 5;
        d *= 2;
    }

    let res: &str = people[(n-1)/d];

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