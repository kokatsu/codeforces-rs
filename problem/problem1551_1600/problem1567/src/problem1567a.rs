use std::io::{stdout, Write, BufWriter};

fn domino(x: char) -> String {
    if x == 'U' {
        return "D".to_string();
    }
    else if x == 'D' {
        return "U".to_string();
    }
    x.to_string()
}

fn main() {
    let t: usize = read();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let _n: usize = read();
        let s: String = read_string();

        let v: Vec<char> = s.chars().collect();

        let res: String = v.into_iter()
                            .map(domino)
                            .collect::<Vec<String>>()
                            .join("");

        writeln!(out, "{}", &res).unwrap();
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