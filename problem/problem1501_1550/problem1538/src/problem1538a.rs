use std::io::{stdout, Write, BufWriter};

fn main() {
    let t: usize = read();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let n: i64 = read();
        let a: Vec<i64> = read_vec();

        let min_index: i64 =
            a
            .iter()
            .enumerate()
            .min_by(|(_, &x), (_, y)| x.cmp(y))
            .map(|(index, _)| index)
            .unwrap() as i64;

        let max_index: i64 =
            a
            .iter()
            .enumerate()
            .max_by(|(_, &x), (_, y)| x.cmp(y))
            .map(|(index, _)| index)
            .unwrap() as i64;

        let res: i64 =
            (min_index.max(max_index)+1)
            .min((n-min_index).max(n-max_index))
            .min(min_index+n-max_index+1)
            .min(max_index+n-min_index+1);

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