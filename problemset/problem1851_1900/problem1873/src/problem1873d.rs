use std::io::{stdout, Write, BufWriter};

fn main() {
    let t: usize = read();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let input: Vec<usize> = read_vec();
        let _n: usize = input[0];
        let k: usize = input[1];

        let s: String = read_string();

        let pos: Vec<usize> =
            s
            .chars()
            .enumerate()
            .filter(|(_, x)| x == &'B')
            .map(|(i, _)| i)
            .collect();

        let res: usize =
            pos
            .iter()
            .fold((0, usize::MAX), |(res, start), &p| {
                if p.abs_diff(start) < k {
                    (res, start)
                }
                else {
                    (res+1, p)
                }
            }).0;

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