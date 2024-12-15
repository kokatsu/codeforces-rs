use std::io::{stdout, Write, BufWriter};

fn main() {
    let q: usize = read();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..q {
        let s: String = read_string();
        let t: String = read_string();

        let u: usize = s.len();
        let v: usize = t.len();
        let g: usize = gcd(u, v);

        let x: String = s.repeat(v/g);
        let y: String = t.repeat(u/g);

        let res: String =
            if x == y {
                x
            }
            else {
                (-1).to_string()
            };

        writeln!(out, "{}", res).unwrap();
    }
}

#[allow(dead_code)]
fn gcd(mut x: usize, mut y: usize) -> usize {
    while y != 0 {
        (x, y) = (y, x % y);
    }
    x
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