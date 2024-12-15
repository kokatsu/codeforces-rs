use std::io::{stdout, BufWriter, Write};

fn threadlets(a: &[u64]) -> &str {
    let s: u64 = a.iter().sum();
    for i in 0..=3 {
        let n: u64 = i + 3;
        let r: u64 = s % n;
        if r > 0 {
            continue;
        }

        let d: u64 = s / n;
        let mut num: u64 = 0;
        for x in a.iter() {
            num += if x % d == 0 { x / d } else { 100 };
        }

        if num == n {
            return "YES";
        }
    }

    "NO"
}

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let a: Vec<u64> = read_vec();

        let res: &str = threadlets(&a);

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
