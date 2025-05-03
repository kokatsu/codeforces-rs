use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let (n, d): (usize, u32) = {
        let input: Vec<usize> = read_vec();
        (input[0], input[1] as u32)
    };

    let mut a: Vec<u32> = read_vec();

    a.sort();

    let mut r: usize = 0;
    let mut res: u32 = 0;
    for l in 0..n {
        while r < n && a[r] - a[l] <= d {
            r += 1;
        }

        res += (r - l - 1) as u32 * 2;
    }

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
