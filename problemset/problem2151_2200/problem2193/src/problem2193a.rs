use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let (_n, s, x): (usize, u8, u8) = {
            let input = read_vec();
            (input[0] as usize, input[1], input[2])
        };

        let a: Vec<u8> = read_vec();

        let sum: u8 = a.iter().sum();

        let res: &str = if s >= sum && (s - sum).is_multiple_of(x) {
            "YES"
        } else {
            "NO"
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
