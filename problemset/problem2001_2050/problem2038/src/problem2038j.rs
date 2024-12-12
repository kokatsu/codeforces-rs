use std::io::{stdout, Write, BufWriter};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let n: usize = read();

    let mut p: i64 = 0;
    for _ in 0..n {
        let (x, y): (String, i64) = {
            let input: Vec<String> = read_vec();
            (input[0].clone(), input[1].parse::<i64>().ok().unwrap())
        };

        if x == "P" {
            p += y;
        }
        else {
            let res: &str = if y - p > 0 { "YES" } else { "NO" };
            p = 0.max(p-y);

            writeln!(out, "{}", res).unwrap();
        }
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