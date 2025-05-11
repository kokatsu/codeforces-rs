use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let s: String = read_string();

        let res: usize = s
            .chars()
            .fold((0, 0, 0), |(res, a, b), c| match c {
                '(' => (res, a + 1, b),
                '[' => (res, a, b + 1),
                ')' if a > 0 => (res + 1, a - 1, b),
                ']' if b > 0 => (res + 1, a, b - 1),
                _ => (res, a, b),
            })
            .0;

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
