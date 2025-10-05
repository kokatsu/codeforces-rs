use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let (t, s, x): (u32, u32, u32) = {
        let input: Vec<u32> = read_vec();
        (input[0], input[1], input[2])
    };

    let res: &str = if x < t {
        "NO"
    } else if x == t
        || ((x - t) % s == 0 && (x - t) / s > 0)
        || ((x - 1 - t) % s == 0 && (x - 1 - t) / s > 0)
    {
        "YES"
    } else {
        "NO"
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
