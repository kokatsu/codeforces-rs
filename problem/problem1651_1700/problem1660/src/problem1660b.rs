use std::io::{stdout, Write, BufWriter};

fn main() {
    let t: usize = read();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let n: usize = read();
        let mut a: Vec<usize> = read_vec();

        a.sort_by(|x, y| x.cmp(y).reverse());

        let res: &str =
            if n == 1 {
                if a[0] == 1 {
                    "YES"
                }
                else {
                    "NO"
                }
            }
            else {
                if a[0] - a[1] <= 1 {
                    "YES"
                }
                else {
                    "NO"
                }
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