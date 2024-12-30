use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let (n, c): (usize, u32) = {
        let input: Vec<u32> = read_vec();
        (input[0] as usize, input[1])
    };

    let t: Vec<u32> = read_vec();

    let res: usize = (1..n).fold(1, |res, i| {
        match t[i] - t[i-1] <= c {
            true => res + 1,
            false => 1,
        }
    });

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
