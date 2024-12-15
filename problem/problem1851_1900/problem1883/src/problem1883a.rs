use std::io::{stdout, Write, BufWriter};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let s: Vec<u8> = read_string()
            .chars()
            .map(|x| x as u8 - '0' as u8)
            .collect();

        let res: u8 = s
            .into_iter()
            .fold((0, 1), |(res, pos), x| {
                let p: u8 = if pos == 0 { 10 } else { pos };
                let q: u8 = if x == 0 { 10 } else { x };
                (res+p.abs_diff(q)+1, x)
            })
            .0;

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