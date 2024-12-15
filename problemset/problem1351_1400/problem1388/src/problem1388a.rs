use std::io::{stdout, BufWriter, Write};

fn main() {
    let t: usize = read();

    let m: u64 = 30;
    let a: Vec<u64> = vec![36, 40, 44];

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let n: u64 = read();

        if n <= m {
            writeln!(out, "NO").unwrap();
            continue;
        }

        writeln!(out, "YES").unwrap();

        let (x, y): (u64, u64) = if a.contains(&n) {
            (15, n - m - 1)
        } else {
            (14, n - m)
        };

        writeln!(out, "6 10 {} {}", x, y).unwrap();
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
