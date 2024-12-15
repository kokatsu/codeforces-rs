use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let s: String = (0..100)
        .map(|i| if i % 4 < 2 { 'A' } else { 'Z' })
        .collect::<String>();

    let t: usize = read();

    for _ in 0..t {
        let n: usize = read();

        let res: String = if n % 2 == 0 {
            "YES".to_string() + "\n" + &s[0..n]
        } else {
            "NO".to_string()
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
