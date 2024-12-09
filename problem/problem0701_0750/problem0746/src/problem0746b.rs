use std::io::{stdout, Write, BufWriter};

fn main() {
    let n: usize = read();
    let s: Vec<char> = read_string().chars().collect();

    let m: usize = n / 2;

    let mut t: Vec<char> = vec![s[0]; n];
    for i in 0..m {
        if n % 2 == 0 {
            t[m-i-1] = s[i*2];
            t[m+i] = s[i*2+1];
        }
        else {
            t[m-i-1] = s[i*2+1];
            t[m+i+1] = s[(i+1)*2];
        }
    }

    let res: String =
        t
        .iter()
        .fold(String::new(), |res, x| res + &x.to_string());

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