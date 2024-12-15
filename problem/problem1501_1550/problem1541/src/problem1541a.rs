use std::io::{stdout, BufWriter, Write};

fn even(x: i64) -> i64 {
    match x % 2 {
        0 => x - 1,
        _ => x + 1,
    }
}

fn odd(x: i64) -> i64 {
    match (x, x % 2) {
        (1, _) => 3,
        (2, _) => 1,
        (3, _) => 2,
        (_, 0) => x + 1,
        _ => x - 1,
    }
}

fn main() {
    let t: usize = read();

    let evens: Vec<i64> = (1..=100).map(even).collect();
    let odds: Vec<i64> = (1..=100).map(odd).collect();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let n: usize = read();

        if n % 2 == 0 {
            writeln!(
                out,
                "{}",
                evens[0..n]
                    .iter()
                    .fold(String::new(), |x, &y| x + &y.to_string() + " ")
            )
            .unwrap();
        } else {
            writeln!(
                out,
                "{}",
                odds[0..n]
                    .iter()
                    .fold(String::new(), |x, &y| x + &y.to_string() + " ")
            )
            .unwrap();
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
