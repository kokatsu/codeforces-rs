use std::io::{stdout, Write, BufWriter};

fn main() {
    let t: usize = read();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let input: Vec<i64> = read_vec();
        let _n: i64 = input[0];
        let m: i64 = input[1];
        let k: i64 = input[2];
        let h: i64 = input[3];

        let a: Vec<i64> = read_vec();

        let res: i64 =
            a
            .iter()
            .fold(0, |res, x| {
                let d: i64 = (h-x).abs();
                if d > 0 && d % k == 0 && d / k < m {
                    res + 1
                }
                else {
                    res
                }
            });

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