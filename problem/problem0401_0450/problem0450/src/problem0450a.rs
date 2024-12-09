use std::io::{stdout, Write, BufWriter};

fn main() {
    let input: Vec<usize> = read_vec();
    let n: usize = input[0];
    let m: usize = input[1];

    let a: Vec<usize> = read_vec();

    let mut times: usize = 0;
    let mut res: usize = 0;
    for i in 0..n {
        let d: usize = (a[i] + m - 1) / m;
        if d >= times {
            times = d;
            res = i + 1;
        }
    }

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