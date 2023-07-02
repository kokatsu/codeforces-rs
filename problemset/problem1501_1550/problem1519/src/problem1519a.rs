use std::io::{stdout, Write, BufWriter};

fn main() {
    let t: usize = read();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let input: Vec<i64> = read_vec();
        let r: i64 = input[0];
        let b: i64 = input[1];
        let d: i64 = input[2];

        let res: &str =
            if r < b {
                if b <= r * (d + 1) {
                    "YES"
                }
                else {
                    "NO"
                }
            }
            else if r > b {
                if r <= b * (d + 1) {
                    "YES"
                }
                else {
                    "NO"
                }
            }
            else {
                "YES"
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