use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let (w, h, k): (i16, i16, i16) = {
        let input: Vec<i16> = read_vec();
        (input[0], input[1], input[2])
    };

    let res: i16 = (0..k).fold(0, |acc, i| {
        let perimeter: i16 = 2 * (w + h - 8 * i) - 4;
        acc + perimeter
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
