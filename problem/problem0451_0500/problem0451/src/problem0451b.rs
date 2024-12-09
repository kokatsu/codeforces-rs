use std::io::{stdout, Write, BufWriter};

fn main() {
    let n: usize = read();
    let mut a: Vec<i64> = read_vec();

    let mut l: usize = 1;
    let mut r: usize = 1;
    for i in 0..n-1 {
        if a[i] > a[i+1] {
            let mut j: usize = i + 1;
            while j < n - 1 && a[j] > a[j+1] {
                j += 1;
            }

            a[i..=j].reverse();
            l = i + 1;
            r = j + 1;

            break;
        }
    }

    let is_ok: bool = (0..n-1).fold(true, |is_ok, i| is_ok && a[i] < a[i+1]);

    let mut out = BufWriter::new(stdout().lock());

    if is_ok {
        writeln!(out, "yes").unwrap();
        writeln!(out, "{} {}", l, r).unwrap();
    }
    else {
        writeln!(out, "no").unwrap();
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