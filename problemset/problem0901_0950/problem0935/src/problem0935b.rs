use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let _n: usize = read();
    let s: Vec<char> = read_string().chars().collect();

    let res: usize = s
        .iter()
        .enumerate()
        .fold((0, 0, 0), |(res, x, y), (i, &c)| {
            let u: usize = if c == 'U' { 1 } else { 0 };
            let r: usize = if c == 'R' { 1 } else { 0 };
            let c: usize = if i > 0 && x == y && s[i - 1] == s[i] {
                1
            } else {
                0
            };
            (res + c, x + u, y + r)
        })
        .0;

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
