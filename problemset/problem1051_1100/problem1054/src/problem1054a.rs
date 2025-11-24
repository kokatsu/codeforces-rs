use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let (x, y, z, t1, t2, t3): (u32, u32, u32, u32, u32, u32) = {
        let input: Vec<u32> = read_vec();
        (input[0], input[1], input[2], input[3], input[4], input[5])
    };

    let a: u32 = x.abs_diff(y) * t1;
    let b: u32 = x.abs_diff(y) * t2;
    let c: u32 = x.abs_diff(z) * t2;

    let res: &str = if a >= b + c + 3 * t3 { "YES" } else { "NO" };

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
