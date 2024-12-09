use std::io::{stdout, BufWriter, Write};

fn main() {
    let n: usize = read();
    let t: Vec<char> = read_string().chars().collect();

    let res: String = (0..10)
        .scan(0, |cum, i| {
            *cum += i;
            Some(*cum)
        })
        .filter(|x| x < &n)
        .fold(String::new(), |res, i| res + &t[i].to_string());

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
