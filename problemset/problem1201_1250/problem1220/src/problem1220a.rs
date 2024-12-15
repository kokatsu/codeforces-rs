use std::io::{stdout, BufWriter, Write};

fn main() {
    let _n: usize = read();
    let s: String = read_string();

    let (zero, one): (usize, usize) = s.chars().fold((0, 0), |(zero, one), x| {
        if x == 'z' {
            (zero + 1, one)
        } else if x == 'n' {
            (zero, one + 1)
        } else {
            (zero, one)
        }
    });

    let res: String = (0..zero + one).fold(String::new(), |res, i| {
        if i < one {
            res + &1.to_string() + " "
        } else {
            res + &0.to_string() + " "
        }
    });

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
