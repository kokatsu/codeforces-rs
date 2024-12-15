use std::io::{stdout, BufWriter, Write};

fn main() {
    let s: String = read_string();
    let t: String = read_string();

    let v: Vec<char> = s.chars().collect();

    let l: usize = v.len();

    let mut res: usize = 0usize;
    for c in t.chars() {
        if res < l && c == v[res] {
            res += 1usize;
        }
    }

    res += 1usize;

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
