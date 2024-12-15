use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let _n: usize = read();
        let s: String = read_string();

        let (is_ok, num): (bool, i32) = s.chars().rev().fold((true, 0), |(is_ok, num), c| {
            if c == 'Q' {
                if num <= 0 {
                    (false, num)
                } else {
                    (is_ok, num - 1)
                }
            } else {
                (is_ok, num + 1)
            }
        });

        let res: &str = if is_ok && num >= 0 { "Yes" } else { "No" };

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
