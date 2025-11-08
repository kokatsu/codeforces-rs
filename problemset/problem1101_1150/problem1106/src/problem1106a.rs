use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let n: usize = read();
    let m: Vec<Vec<char>> = (0..n).map(|_| read_string().chars().collect()).collect();

    let res: usize = (1..n - 1)
        .map(|i| {
            let mut c: usize = 0;
            for j in 1..n - 1 {
                if m[i][j] == 'X'
                    && m[i - 1][j - 1] == 'X'
                    && m[i + 1][j - 1] == 'X'
                    && m[i - 1][j + 1] == 'X'
                    && m[i + 1][j + 1] == 'X'
                {
                    c += 1;
                }
            }
            c
        })
        .sum();

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
