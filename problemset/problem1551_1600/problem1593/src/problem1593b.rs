use std::io::{stdout, Write, BufWriter};

fn solve(s: &[char], a: char, b: char) -> usize {
    let l: usize = s.len();

    if let Some(y) = s.iter().position(|&c| c == b) {
        if let Some(x) = s[y+1..l].iter().position(|&c| c == a) {
            return x + y;
        }
    }

    l
}

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let n: Vec<char> = read_string().chars().rev().collect();

        let n00: usize = solve(&n, '0', '0');
        let n25: usize = solve(&n, '2', '5');
        let n50: usize = solve(&n, '5', '0');
        let n75: usize = solve(&n, '7', '5');

        let res: usize = [n00, n25, n50, n75].into_iter().min().unwrap();

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