use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let (a, b, c, d): (u32, u32, u32, u32) = {
        let input: Vec<u32> = read_vec();
        (input[0], input[1], input[2], input[3])
    };

    let res: u32 = {
        let mut v: Vec<u32> = vec![a, b, c];
        v.sort();
        d.saturating_sub(v[1] - v[0]) + d.saturating_sub(v[2] - v[1])
    };

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
