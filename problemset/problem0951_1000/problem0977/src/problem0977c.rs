use std::io::{stdout, Write, BufWriter};

fn main() {
    let input: Vec<usize> = read_vec();
    let n: usize = input[0];
    let k: usize = input[1];

    let mut a: Vec<i64> = read_vec();

    a.sort();

    let res: i64 =
        if k == n {
            a[k-1]
        }
        else if k == 0 {
            if a[0] == 1 {
                -1
            }
            else {
                1
            }
        }
        else if a[k-1] == a[k] {
            -1
        }
        else {
            a[k-1]
        };

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