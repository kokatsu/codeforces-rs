use std::io::{stdout, Write, BufWriter};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let n: usize = read();

        let mut d: Vec<i64> = vec![0; n * 2 + 1];
        for i in 0..n {
            let a: Vec<i64> = read_vec();

            for (j, &x) in a.iter().enumerate() {
                let k: usize = n + i - j - 1;
                d[k] = d[k].min(x);
            }
        }

        let res: i64 = d
            .iter()
            .map(|x| x.abs())
            .sum();

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