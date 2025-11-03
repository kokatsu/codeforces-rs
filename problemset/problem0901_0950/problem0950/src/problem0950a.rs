use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let (l, r, a): (u16, u16, u16) = {
        let input: Vec<u16> = read_vec();
        (input[0], input[1], input[2])
    };

    let d: u16 = l.abs_diff(r);
    let c: u16 = if a >= d { d + (a - d) / 2 } else { a };

    let res: u16 = (l.min(r) + c) * 2;

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
