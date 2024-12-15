use std::io::{stdout, Write, BufWriter};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let q: usize = read();

    for _ in 0..q {
        let s: Vec<char> = read_string().chars().collect();
        let t: Vec<char> = read_string().chars().collect();

        let u: usize = s.len();
        let v: usize = t.len();
        let m: usize = u.min(v);

        let n: usize = (0..m)
            .find(|&i| s[i] != t[i])
            .unwrap_or(m);

        let w = if n == 0 { n } else { n - 1 };

        let res: usize = u + v - w ;

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