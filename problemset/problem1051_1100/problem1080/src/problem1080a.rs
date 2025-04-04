use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let (n, k): (u32, u32) = {
        let input: Vec<u32> = read_vec();
        (input[0], input[1])
    };

    let r: u32 = n * 2;
    let g: u32 = n * 5;
    let b: u32 = n * 8;

    let res: u32 = r.div_ceil(k) + g.div_ceil(k) + b.div_ceil(k);

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
