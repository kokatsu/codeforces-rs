use std::io::{stdout, Write, BufWriter};

fn main() {
    let n: usize = read();

    let mut a: Vec<(u64, u64)> = vec![(0, 0); n];
    for i in 0..n {
        let input: Vec<u64> = read_vec();
        a[i] = (input[0], input[1]);
    }

    a.sort();

    let res: u64 =
        a
        .iter()
        .fold(0, |res, (x, y)| {
            if res <= *y {
                *y
            }
            else {
                *x
            }
        });

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