use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let (x1, y1): (u32, u32) = {
            let input: Vec<u32> = read_vec();
            (input[0], input[1])
        };

        let (x2, y2): (u32, u32) = {
            let input: Vec<u32> = read_vec();
            (input[0], input[1])
        };

        let res: &str = if (x1 < y1) == (x2 < y2) { "YES" } else { "NO" };

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
