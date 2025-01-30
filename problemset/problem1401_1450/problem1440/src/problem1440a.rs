use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let (n, c0, c1, h): (usize, usize, usize, usize) = {
            let input: Vec<usize> = read_vec();
            (input[0], input[1], input[2], input[3])
        };

        let s: String = read_string();

        let zero: usize = c0.min(c1 + h);
        let one: usize = c1.min(c0 + h);

        let z: usize = s.chars().filter(|&x| x == '0').count();

        let res: usize = z * zero + (n - z) * one;

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
