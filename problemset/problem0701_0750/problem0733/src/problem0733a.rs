use std::io::{stdout, BufWriter, Write};

const VOWELS: &str = "AEIOUY";

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let mut s: Vec<char> = read_string().chars().collect();

    s.insert(0, 'A');
    s.push('A');

    let l: usize = s.len();

    let d: Vec<usize> = (0..l).filter(|&i| VOWELS.contains(s[i])).collect();
    let n: usize = d.len();

    let res: usize = (0..n - 1).map(|i| d[i + 1] - d[i]).max().unwrap();

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
