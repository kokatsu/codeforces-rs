use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let _n: usize = read();
        let a: Vec<u32> = read_vec();

        let s: u32 = a.iter().sum();

        let res: u32 = if s % 2 == 0 {
            0
        } else {
            a.iter()
                .map(|&x| {
                    let mut y: u32 = x;
                    let mut m: u32 = 0;
                    while y % 2 == x % 2 {
                        y /= 2;
                        m += 1;
                    }
                    m
                })
                .min()
                .unwrap()
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
