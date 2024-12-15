use std::io::{stdout, BufWriter, Write};

fn main() {
    let t: usize = read();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let input: Vec<i64> = read_vec();
        let _n: i64 = input[0];
        let l: i64 = input[1];
        let r: i64 = input[2];

        let mut a: Vec<i64> = read_vec();

        a.sort();

        let res: u64 = a.iter().enumerate().fold(0, |res, (i, x)| {
            let lower: usize = a.partition_point(|&v| v < l - x);
            let upper: usize = a.partition_point(|&v| v <= r - x);
            if lower <= i && i < upper {
                res + upper as u64 - lower as u64 - 1
            } else {
                res + upper as u64 - lower as u64
            }
        }) / 2;

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
