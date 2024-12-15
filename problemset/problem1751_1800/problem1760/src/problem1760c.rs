use std::io::{stdout, BufWriter, Write};

fn main() {
    let t: usize = read();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let n: usize = read();
        let s: Vec<i64> = read_vec();

        let mut x: Vec<(i64, usize)> = (0..n).map(|x| (s[x], x)).collect();
        x.sort_by(|a, b| (-a.0).partial_cmp(&(-b.0)).unwrap());

        for (i, a) in s.iter().enumerate() {
            let res: i64 = {
                if i == x[0].1 {
                    a - x[1].0
                } else {
                    a - x[0].0
                }
            };

            if i < n - 1 {
                write!(out, "{} ", res).unwrap();
            } else {
                writeln!(out, "{}", res).unwrap();
            }
        }
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
