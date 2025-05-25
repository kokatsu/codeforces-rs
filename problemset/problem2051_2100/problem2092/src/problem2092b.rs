use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let n: usize = read();
        let a: Vec<char> = read_string().chars().collect();
        let b: Vec<char> = read_string().chars().collect();

        let (c, d): (usize, usize) = (0..n).fold((0, 0), |(c, d), i| {
            let x: usize = if a[i] == '0' { 1 } else { 0 };
            let y: usize = if b[i] == '0' { 1 } else { 0 };
            if i % 2 == 0 {
                (c + x, d + y)
            } else {
                (c + y, d + x)
            }
        });

        let res: &str = if c >= n.div_ceil(2) && d >= n / 2 {
            "YES"
        } else {
            "NO"
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
