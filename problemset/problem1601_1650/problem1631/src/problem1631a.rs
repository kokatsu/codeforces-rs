use std::io::{stdout, BufWriter, Write};

fn main() {
    let t: usize = read();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let _n: usize = read();
        let a: Vec<i64> = read_vec();
        let b: Vec<i64> = read_vec();

        let (x, y): (i64, i64) = a.iter().zip(b.iter()).fold((-1, -1), |(x, y), (&u, &v)| {
            if u < v {
                (x.max(u), y.max(v))
            } else {
                (x.max(v), y.max(u))
            }
        });

        let res: i64 = x * y;

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
