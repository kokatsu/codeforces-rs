use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let (r, x, y, u, v): (f64, f64, f64, f64, f64) = {
        let input: Vec<f64> = read_vec();
        (input[0], input[1], input[2], input[3], input[4])
    };

    let d: f64 = (x - u).hypot(y - v);

    let res: u64 = (d / (2.0 * r)).ceil() as u64;

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
