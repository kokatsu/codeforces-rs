use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: Vec<char> = read_string().chars().collect();

    let l: usize = t.len();
    let m: usize = l % 2;

    let mut s: Vec<char> = Vec::new();

    for i in 0..l {
        if (i % 2) == m {
            s.push(t[l - 1 - i / 2]);
        } else {
            s.push(t[i / 2]);
        }
    }

    s.reverse();

    let res: String = s.iter().collect();

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
