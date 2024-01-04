use std::io::{stdout, Write, BufWriter};

fn main() {
    let t: usize = read();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let input: Vec<i64> = read_vec();
        let (px, py): (i64, i64) = (input[0], input[1]);

        let s: String = read_string();

        let (u, d, r, l): (i64, i64, i64, i64) = s
            .chars()
            .fold((0, 0, 0, 0), |(u, d, r, l), c| {
                if c == 'U' {
                    (u+1, d, r, l)
                }
                else if c == 'D' {
                    (u, d-1, r, l)
                }
                else if c == 'R' {
                    (u, d, r+1, l)
                }
                else {
                    (u, d, r, l-1)
                }
            });

        let okx: bool = if px >= 0 { r >= px } else { l <= px };
        let oky: bool = if py >= 0 { u >= py } else { d <= py };

        let res: &str = if okx && oky { "YES" } else { "NO" };

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