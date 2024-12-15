use std::io::{stdout, BufWriter, Write};

fn main() {
    let n: usize = read();

    let mut a: Vec<u64> = vec![0; n];
    for i in 0..n {
        a[i] = read();
    }

    let s: u64 = a.iter().sum();

    let l: usize = 1 << n;
    let mut res: &str = "NO";
    for i in 0..l {
        let b: u64 = (0..n).fold(0, |b, j| if (i >> j) % 2 == 1 { b + a[j] } else { b });

        if b * 2 == s || b.abs_diff(s - b) % 360 == 0 {
            res = "YES";
        }
    }

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
