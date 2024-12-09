use std::io::{stdout, Write, BufWriter};

fn main() {
    let n: i64 = read();

    let limit: usize = 10;

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..n {
        let s: String = read_string();

        let l: usize = s.len();

        let res: String =
            if l > limit {
                format!("{}{}{}", s.chars().nth(0).unwrap(), l-2, s.chars().nth(l-1).unwrap())
            }
            else {
                s
            };

        writeln!(out, "{}", &res).unwrap();
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