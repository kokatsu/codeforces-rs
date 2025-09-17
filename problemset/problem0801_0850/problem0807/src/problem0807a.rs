use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let n: usize = read();

    let mut a: Vec<u16> = vec![0; n];
    let mut b: Vec<u16> = vec![0; n];
    let mut d: bool = false;
    for i in 0..n {
        let input: Vec<u16> = read_vec();
        a[i] = input[0];
        b[i] = input[1];
        if a[i] != b[i] {
            d = true;
        }
    }

    let res: &str = if d {
        "rated"
    } else if a.windows(2).any(|w| w[0] < w[1]) {
        "unrated"
    } else {
        "maybe"
    };

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
