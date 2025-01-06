use std::cmp::Ordering;
use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let (s, v1, v2, t1, t2): (u32, u32, u32, u32, u32) = {
        let input: Vec<u32> = read_vec();
        (input[0], input[1], input[2], input[3], input[4])
    };

    let first: u32 = v1 * s + t1 * 2;
    let second: u32 = v2 * s + t2 * 2;

    let res: &str = match first.cmp(&second) {
        Ordering::Less => "First",
        Ordering::Greater => "Second",
        Ordering::Equal => "Friendship",
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
