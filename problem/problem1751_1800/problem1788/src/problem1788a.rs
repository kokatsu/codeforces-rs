use std::io::{stdout, Write, BufWriter};

fn main() {
    let t: usize = read();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let n: usize = read();
        let a: Vec<u64> = read_vec();

        let cumsum: Vec<u64> =
            a
            .iter()
            .scan(0, |cumsum, x| {
                *cumsum += x / 2;
                Some(*cumsum)
            })
            .collect();

        let index: Option<usize> =
            (0..n-1)
            .find(|&i| cumsum[i] * 2 == cumsum[n-1]);

        let res: i64 = match index {
            Some(index) => index as i64 + 1,
            None => -1,
        };

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