use std::io::{stdout, Write, BufWriter};

fn main() {
    let _n: usize = read();
    let a: Vec<i64> = read_vec();

    let (num, zero, minus): (i64, bool, bool) =
        a
        .iter()
        .fold((0, false, false), |(num, zero, minus), &x| {
            if x > 0 {
                (num+x-1, zero, minus)
            }
            else if x < 0 {
                (num-x-1, zero, !minus)
            }
            else {
                (num+1, true, minus)
            }
        });

    let res: i64 =
        if zero || !minus {
            num
        }
        else {
            num + 2
        };

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