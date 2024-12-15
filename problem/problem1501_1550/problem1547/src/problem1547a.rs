use std::io::{stdout, BufWriter, Write};

fn main() {
    let t: usize = read();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        read_string();

        let a: Vec<i64> = read_vec();
        let x_a: i64 = a[0];
        let y_a: i64 = a[1];

        let b: Vec<i64> = read_vec();
        let x_b: i64 = b[0];
        let y_b: i64 = b[1];

        let f: Vec<i64> = read_vec();
        let x_f: i64 = f[0];
        let y_f: i64 = f[1];

        let res: i64 = if x_a == x_b
            && x_a == x_f
            && ((y_a <= y_f && y_f <= y_b) || (y_b <= y_f && y_f <= y_a))
        {
            (y_a - y_b).abs() + 2
        } else if y_a == y_b
            && y_a == y_f
            && ((x_a <= x_f && x_f <= x_b) || (x_b <= x_f && x_f <= x_a))
        {
            (x_a - x_b).abs() + 2
        } else {
            (x_a - x_b).abs() + (y_a - y_b).abs()
        };

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
