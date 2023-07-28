use std::io::{stdout, Write, BufWriter};

fn main() {
    let d: Vec<u64> = read_vec();

    let res: u64 =
        *vec![(d[0]+d[1])*2, (d[0]+d[2])*2, (d[1]+d[2])*2, d.iter().sum::<u64>()]
        .iter()
        .min()
        .unwrap();

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