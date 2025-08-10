use std::io::{stdout, BufWriter, Write};

fn get_last_chars(x: &str, y: usize) -> &str {
    let len = x.len();
    if len <= y {
        return x;
    }
    &x[len - y..]
}

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let s: String = read_string();

    let t: &str = get_last_chars(&s, 2);
    let n: usize = t.parse().unwrap();

    let res: usize = if n % 4 == 0 { 4 } else { 0 };

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
