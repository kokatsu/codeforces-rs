use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let (x1, y1): (i32, i32) = {
        let input: Vec<i32> = read_vec();
        (input[0], input[1])
    };

    let (x2, y2): (i32, i32) = {
        let input: Vec<i32> = read_vec();
        (input[0], input[1])
    };

    let res: i32 = (x2 - x1).abs().max((y2 - y1).abs());

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
