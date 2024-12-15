use std::io::{stdout, BufWriter, Write};

fn main() {
    let input: Vec<usize> = read_vec();
    let r: usize = input[0];
    let c: usize = input[1];

    let s: Vec<Vec<char>> = (0..r)
        .map(|_| read_string().chars().collect::<Vec<char>>())
        .collect();

    let x: usize = (0..r).fold(0, |x, i| match s[i].iter().find(|&&u| u == 'S') {
        Some(_) => x,
        None => x + 1,
    });

    let y: usize = (0..c).fold(0, |y, j| {
        match (0..r).map(|i| s[i][j]).find(|&u| u == 'S') {
            Some(_) => y,
            None => y + 1,
        }
    });

    let res: usize = c * x + (r - x) * y;

    let mut out = BufWriter::new(stdout().lock());
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
