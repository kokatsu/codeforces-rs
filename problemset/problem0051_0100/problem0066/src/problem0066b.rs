use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let n: usize = read();
    let a: Vec<u16> = read_vec();

    let res: usize = (0..n)
        .map(|i| {
            let mut l: usize = i;
            while l > 0 && a[l - 1] <= a[l] {
                l -= 1;
            }
            let mut r: usize = i;
            while r < n - 1 && a[r] >= a[r + 1] {
                r += 1;
            }
            r - l + 1
        })
        .max()
        .unwrap_or(1);

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
