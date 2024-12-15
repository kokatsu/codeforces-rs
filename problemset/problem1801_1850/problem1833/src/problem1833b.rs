use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let input: Vec<usize> = read_vec();
        let (n, _k): (usize, i64) = (input[0], input[1] as i64);

        let a: Vec<i64> = read_vec();
        let mut b: Vec<i64> = read_vec();

        let mut c: Vec<(i64, usize)> = (0..n).map(|i| (a[i], i)).collect();

        b.sort();
        c.sort();

        let mut d: Vec<i64> = vec![0; n];
        for i in 0..n {
            d[c[i].1] = b[i];
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
