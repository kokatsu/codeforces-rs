use std::io::{stdout, BufWriter, Write};

fn main() {
    let t: usize = read();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let n: usize = read();
        let a: Vec<u64> = read_vec();

        if a[0] != a[n - 1] {
            let res: String = (1..n)
                .rev()
                .fold(a[0].to_string(), |res, i| res + " " + &a[i].to_string());
            writeln!(out, "YES\n{}", res).unwrap();
        } else {
            writeln!(out, "NO").unwrap();
        }
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
