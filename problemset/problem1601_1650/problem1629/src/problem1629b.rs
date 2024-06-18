use std::io::{stdout, Write, BufWriter};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let (l, r, k): (u64, u64, u64) = {
            let input: Vec<u64> = read_vec();
            (input[0], input[1], input[2])
        };

        let res: &str =
            if l == r {
                if l > 1 { "YES" } else { "NO" }
            }
            else {
                let d: u64 = r / 2 - (l - 1) / 2 + k;
                let s: u64 = r - l + 1;
                if d >= s { "YES" } else { "NO" }
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