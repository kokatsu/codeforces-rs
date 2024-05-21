use std::io::{stdout, Write, BufWriter};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let (x, y): (i64, i64) = {
            let input: Vec<i64> = read_vec();
            (input[0], input[1])
        };

        let s: i64 = x * x + y * y;
        let r: i64 = (s as f64).sqrt().floor() as i64;

        let res: i64 =
            if s == 0 {
                0
            }
            else if r * r == s {
                1
            }
            else {
                2
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