use std::io::{stdout, BufWriter, Write};

fn main() {
    let t: usize = read();

    let pyramid: Vec<u64> = (1..100_000).map(|i| i * (i * 3 + 1) / 2).collect();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let mut n: u64 = read();

        let mut res: u64 = 0;
        while n > 1 {
            let p: usize = pyramid.partition_point(|&v| v <= n);
            res += 1;
            n -= pyramid[p - 1];
        }

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
