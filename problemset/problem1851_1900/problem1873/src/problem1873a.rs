use std::io::{stdout, BufWriter, Write};

fn main() {
    let t: usize = read();

    let list: Vec<&str> = vec!["abc", "acb", "bac", "cba"];

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let s: String = read_string();

        let res: &str = match list.iter().find(|&&x| x == s) {
            Some(_) => "YES",
            None => "NO",
        };

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
