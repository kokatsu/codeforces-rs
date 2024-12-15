use std::io::{stdout, Write, BufWriter};
use std::iter::repeat;

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let (n, k): (usize, usize) = {
            let input: Vec<usize> = read_vec();
            (input[0], input[1])
        };

        let res: String =
            if n == k {
                repeat("1".to_string())
                    .take(n)
                    .collect::<Vec<String>>()
                    .join(" ")
            }
            else if k == 1 {
                (1..=n)
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            }
            else {
                "-1".to_string()
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