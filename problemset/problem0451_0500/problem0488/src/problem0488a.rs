use std::io::{stdout, BufWriter, Write};

fn is_lacky(x: i32) -> bool {
    let s: String = x.abs().to_string();
    s.chars().any(|c| c == '8')
}

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let a: i32 = read();

    for i in 1..=30 {
        if is_lacky(a + i) {
            let res: i32 = i;
            writeln!(out, "{}", res).unwrap();
            return;
        }
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
