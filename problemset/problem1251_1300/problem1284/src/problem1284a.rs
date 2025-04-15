use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let (n, m): (usize, usize) = {
        let input: Vec<usize> = read_vec();
        (input[0], input[1])
    };

    let s: Vec<String> = read_vec();
    let t: Vec<String> = read_vec();

    let q: usize = read();

    for _ in 0..q {
        let y: usize = read();

        let a: usize = (y + n - 1) % n;
        let b: usize = (y + m - 1) % m;

        let res: String = format!("{}{}", s[a], t[b]);

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
