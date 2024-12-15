use std::io::{stdout, Write, BufWriter};
use std::cmp::Ordering::{Less, Greater};

fn compare(x: char, y: char) -> char {
    match x.cmp(&y) {
        Less => '<',
        Greater => '>',
        _ => '=',
    }
}

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let s: Vec<char> = read_string().chars().collect();

        let op: char = compare(s[0], s[2]);

        let res: String = [s[0], op, s[2]]
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("");

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