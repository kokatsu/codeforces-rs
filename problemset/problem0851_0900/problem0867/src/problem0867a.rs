use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let n: usize = read();
    let s: Vec<char> = read_string().chars().collect();

    let m: i32 = (1..n).fold(0, |m, i| match (s[i - 1], s[i]) {
        ('S', 'F') => m + 1,
        ('F', 'S') => m - 1,
        _ => m,
    });

    let res: &str = if m > 0 { "YES" } else { "NO" };

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
