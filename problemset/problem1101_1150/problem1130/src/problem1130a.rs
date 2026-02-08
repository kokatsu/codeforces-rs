use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let n: usize = read();
    let a: Vec<i16> = read_vec();

    let (pos, neg): (usize, usize) = a.iter().fold((0, 0), |(p, q), &x| {
        if x > 0 {
            (p + 1, q)
        } else if x < 0 {
            (p, q + 1)
        } else {
            (p, q)
        }
    });

    let res: i16 = match n.div_ceil(2) {
        x if x <= pos => 1,
        x if x <= neg => -1,
        _ => 0,
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
