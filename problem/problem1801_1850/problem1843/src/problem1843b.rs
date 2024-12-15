use std::io::{stdout, BufWriter, Write};

fn main() {
    let t: usize = read();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let _n: usize = read();
        let a: Vec<i64> = read_vec();

        let sum: i64 = a.iter().fold(0, |sum, x| sum + x.abs());

        let cnt: i64 = a
            .iter()
            .fold((0, false), |(cnt, is_replace), &x| {
                if x < 0 && !is_replace {
                    (cnt + 1, true)
                } else if x > 0 {
                    (cnt, false)
                } else {
                    (cnt, is_replace)
                }
            })
            .0;

        writeln!(out, "{} {}", sum, cnt).unwrap();
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
