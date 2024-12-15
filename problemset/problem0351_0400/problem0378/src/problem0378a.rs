use std::cmp::Ordering;
use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let (a, b): (u8, u8) = {
        let input: Vec<u8> = read_vec();
        (input[0], input[1])
    };

    let (first, draw, second): (u8, u8, u8) =
        (1..=6).fold((0, 0, 0), |(first, draw, second), i| {
            let x: u8 = a.abs_diff(i);
            let y: u8 = b.abs_diff(i);
            match x.cmp(&y) {
                Ordering::Less => (first + 1, draw, second),
                Ordering::Equal => (first, draw + 1, second),
                _ => (first, draw, second + 1),
            }
        });

    let res: String = format!("{} {} {}", first, draw, second);

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
