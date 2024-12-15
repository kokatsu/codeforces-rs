use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let (n, m): (usize, usize) = {
            let input: Vec<usize> = read_vec();
            (input[0], input[1])
        };

        let s: Vec<char> = read_string().chars().collect();
        let t: Vec<char> = read_string().chars().collect();

        let u: usize = (1..n).filter(|&i| s[i] == s[i - 1]).count();
        let v: usize = (1..m).filter(|&i| t[i] == t[i - 1]).count();
        let w: usize = if s[n - 1] == t[m - 1] { 1 } else { 0 };

        let res = if u + v + w < 2 { "YES" } else { "NO" };

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
