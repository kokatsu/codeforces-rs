use std::io::{stdout, BufWriter, Write};

fn main() {
    let t: usize = read();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let n: usize = read();
        let mut a: Vec<i64> = read_vec();

        a.sort();

        let mut d: Vec<i64> = vec![0; n];
        for i in 0..(n + 1) / 2 {
            d[n - i * 2 - 1] = a[i];
            if i != n - i - 1 {
                d[n - i * 2 - 2] = a[n - i - 1];
            }
        }

        let res: String = (1..n).fold(d[0].to_string(), |res, i| res + " " + &d[i].to_string());

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
