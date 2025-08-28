use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let (n, m): (usize, usize) = {
        let input: Vec<usize> = read_vec();
        (input[0], input[1])
    };

    let s: Vec<Vec<char>> = (0..n).map(|_| read_string().chars().collect()).collect();

    let a: Vec<u32> = read_vec();

    let res: u32 = (0..m).fold(0, |acc, j| {
        let mut cnt: [u32; 26] = [0; 26];
        for item in s.iter().take(n) {
            let c: usize = (item[j] as u8 - b'A') as usize;
            cnt[c] += 1;
        }
        acc + cnt.iter().max().unwrap() * a[j]
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
