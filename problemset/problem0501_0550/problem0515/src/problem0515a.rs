use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let (a, b, s): (i32, i32, i32) = {
        let input: Vec<i32> = read_vec();
        (input[0], input[1], input[2])
    };

    let x: i32 = a.abs();
    let y: i32 = b.abs();

    let res: &str = if (s >= x + y) && ((s - x - y) % 2 == 0) {
        "Yes"
    } else {
        "No"
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
