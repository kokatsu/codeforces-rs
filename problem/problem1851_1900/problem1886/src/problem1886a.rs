use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let n: u64 = read();

        if n < 7 || n == 9 {
            writeln!(out, "NO").unwrap();
            continue;
        }

        writeln!(out, "YES").unwrap();

        if n % 3 == 0 {
            writeln!(out, "1 4 {}", n - 5).unwrap();
        } else {
            writeln!(out, "1 2 {}", n - 3).unwrap();
        }
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
