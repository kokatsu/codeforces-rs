use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let n: u64 = read();
        let a: Vec<u64> = read_vec();

        let s: u64 = a.iter().sum();

        let even: u64 = (0..n as usize).step_by(2).map(|i| a[i]).sum();
        let odd: u64 = s - even;

        let lo: u64 = n / 2;
        let le: u64 = n - lo;

        let res: &str = if s % n == 0 && even % le == 0 && odd % lo == 0 && even / le == odd / lo {
            "YES"
        } else {
            "NO"
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
