use std::cmp::Ordering;
use std::io::{stdout, BufWriter, Write};

fn calculate_score(p: u16, t: u16) -> u16 {
    (3 * p / 10).max(p - (p / 250) * t)
}

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let (a, b, c, d): (u16, u16, u16, u16) = {
        let input: Vec<u16> = read_vec();
        (input[0], input[1], input[2], input[3])
    };

    let misha: u16 = calculate_score(a, c);
    let vasya: u16 = calculate_score(b, d);

    let res: &str = match misha.cmp(&vasya) {
        Ordering::Greater => "Misha",
        Ordering::Less => "Vasya",
        Ordering::Equal => "Tie",
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
