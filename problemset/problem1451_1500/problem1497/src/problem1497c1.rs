use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let input: Vec<u64> = read_vec();
        let (n, _k): (u64, u64) = (input[0], input[1]);

        let (div, rem): (u64, u64) = (n / 2, n % 2);

        let (a1, a2, a3): (u64, u64, u64) = if rem % 2 == 1 {
            (div, div, 1)
        } else if div % 2 == 0 {
            (div, div / 2, div / 2)
        } else {
            (div - 1, div - 1, 2)
        };

        writeln!(out, "{} {} {}", a1, a2, a3).unwrap();
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
