use std::io::{stdout, Write, BufWriter};

fn main() {
    let n: usize = read();

    let mut counts: Vec<u64> = vec![0; 2400_usize];

    for _ in 0..n {
        let input: Vec<usize> = read_vec();
        let (h, m): (usize, usize) = (input[0], input[1]);

        let index: usize = h * 100 + m;
        counts[index] += 1;
    }

    let res: &u64 = counts.iter().max().unwrap();

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