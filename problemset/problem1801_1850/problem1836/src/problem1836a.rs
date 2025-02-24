use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let n: usize = read();
        let l: Vec<usize> = read_vec();

        let mut is_ok: bool = true;
        let mut v: Vec<usize> = vec![0; n];
        for &x in l.iter() {
            if x < n {
                v[x] += 1;
            } else {
                is_ok = false;
            }
        }

        let is_ok: bool = is_ok && (1..n).all(|i| v[i - 1] >= v[i]);

        let res: &str = if is_ok { "YES" } else { "NO" };

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
