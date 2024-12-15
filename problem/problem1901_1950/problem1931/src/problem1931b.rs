use std::io::{stdout, Write, BufWriter};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let n: i64 = read();
        let a: Vec<i64> = read_vec();

        let m: i64 = a.iter().sum::<i64>() / n;

        let res: &str = a
            .iter()
            .rev()
            .fold(("YES", 0_i64), |(res, num), x| {
                let next: i64 = num + m - x;
                if next >= 0 {
                    (res, next)
                }
                else {
                    ("NO", next)
                }
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