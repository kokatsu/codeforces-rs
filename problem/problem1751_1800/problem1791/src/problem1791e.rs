use std::io::{stdout, Write, BufWriter};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let _n: usize = read();
        let a: Vec<i64> = read_vec();

        let (sum, min, negative_counts, exsits_zero): (i64, i64, usize, bool) = a
            .iter()
            .fold((0, i64::MAX, 0, false), |(sum, min, negative_counts, exsits_zero), &x| {
                let count: usize = if x < 0 { 1 } else { 0 };
                let is_zero: bool = x == 0;
                (sum+x.abs(), min.min(x.abs()), negative_counts+count, exsits_zero|is_zero)
            });

        let res: i64 =
            if negative_counts % 2 == 1 && !exsits_zero {
                sum - min * 2
            }
            else {
                sum
            };

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