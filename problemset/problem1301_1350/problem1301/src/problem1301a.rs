use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let a: Vec<char> = read_string().chars().collect();
        let b: Vec<char> = read_string().chars().collect();
        let c: String = read_string();

        let is_ok: bool =
            c.chars()
                .enumerate()
                .fold(true, |is_ok: bool, (i, x)| match (x == a[i], x == b[i]) {
                    (true, _) | (_, true) => is_ok,
                    _ => false,
                });

        let res: &str = if is_ok { "YES" } else { "NO" };

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
