use std::io::{stdout, Write, BufWriter};

fn main() {
    let t: usize = read();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let input: Vec<u64> = read_vec();
        let _n: u64 = input[0];
        let m: u64 = input[1];

        let a: Vec<u64> = read_vec();

        let res: u64 =
            a
            .iter()
            .fold((0, m), |(res, rem), &x| {
                if rem >= x {
                    (res, rem-x)
                }
                else {
                    let d: u64 = x - rem;
                    (res+d, 0)
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