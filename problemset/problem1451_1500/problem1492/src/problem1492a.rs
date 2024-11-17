use std::io::{stdout, Write, BufWriter};

fn calc(x: u64, y: u64) -> u64 {
    let m: u64 = (x + y - 1) / y;
    m * y - x
}

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let (p, a, b, c): (u64, u64, u64, u64) = {
            let input: Vec<u64> = read_vec();
            (input[0], input[1], input[2], input[3])
        };

        let res: u64 = calc(p, a)
            .min(calc(p, b))
            .min(calc(p, c));

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