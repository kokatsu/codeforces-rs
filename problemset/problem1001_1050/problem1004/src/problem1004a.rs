use std::cmp::Ordering::{Equal, Greater, Less};
use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let (n, d): (usize, i32) = {
        let input: Vec<i32> = read_vec();
        (input[0] as usize, input[1])
    };

    let x: Vec<i32> = read_vec();

    let res: usize = (0..n - 1).fold(2, |res, i| match (x[i + 1] - x[i]).abs().cmp(&(d * 2)) {
        Greater => res + 2,
        Equal => res + 1,
        Less => res,
    });

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
