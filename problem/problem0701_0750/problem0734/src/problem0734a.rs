use std::io::{stdout, Write, BufWriter};

fn main() {
    let _n: usize = read();
    let s: String = read_string();

    let count: i64 =
        s
        .chars()
        .fold(0, |count, x| {
            if x == 'A' {
                count + 1
            }
            else {
                count - 1
            }
        });

    let res: &str =
        if count > 0 {
            "Anton"
        }
        else if count < 0 {
            "Danik"
        }
        else {
            "Friendship"
        };

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