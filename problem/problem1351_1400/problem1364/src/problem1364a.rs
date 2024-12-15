use std::io::{stdout, BufWriter, Write};

fn main() {
    let t: usize = read();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let input: Vec<usize> = read_vec();
        let n: usize = input[0];
        let x: usize = input[1];

        let a: Vec<usize> = read_vec();

        let res: i64 = if a.iter().all(|&v| v % x == 0) {
            -1
        } else if a.iter().sum::<usize>() % x != 0 {
            n as i64
        } else {
            let l: usize = (0..n).find(|&i| a[i] % x != 0).unwrap();

            let r: usize = (0..n).rev().find(|&i| a[i] % x != 0).unwrap();

            (n - l - 1).max(r) as i64
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
