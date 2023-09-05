use std::io::{stdout, Write, BufWriter};

fn main() {
    let t: usize = read();

    let y: String = String::from("2020");

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let _n: usize = read();
        let s: String = read_string();

        let res: &str =
            (0..=4)
            .fold("NO", |res, i| {
                if s.starts_with(&y[0..i]) && s.ends_with(&y[i..4]) {
                    "YES"
                }
                else{
                    res
                }
            });

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