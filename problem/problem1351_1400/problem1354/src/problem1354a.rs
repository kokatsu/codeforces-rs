use std::io::{stdout, Write, BufWriter};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let (a, b, c, d): (i64, i64, i64, i64) = {
            let input: Vec<i64> = read_vec();
            (input[0], input[1], input[2], input[3])
        };

        let res: i64 =
            if a <= b {
                b
            }
            else if c > d {
                let s: i64 = c - d;
                b + (a - b + s - 1) / s * c
            }
            else {
                -1
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