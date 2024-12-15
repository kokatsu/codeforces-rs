use std::io::{stdout, Write, BufWriter};

fn main() {
    let input: Vec<usize> = read_vec();
    let (_n, q): (usize, usize) = (input[0], input[1]);

    let mut p: Vec<u64> = read_vec();
    p.push(u64::MAX);
    p.sort_by(|x, y| x.cmp(y).reverse());
    p[0] = 0;

    let cum: Vec<u64> = p
        .iter()
        .scan(0, |cum, x| {
            *cum += x;
            Some(*cum)
        })
        .collect();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..q {
        let input: Vec<usize> = read_vec();
        let (x, y): (usize, usize) = (input[0], input[1]);

        let res: u64 = cum[x] - cum[x-y];

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