use std::io::{stdout, BufWriter, Write};

fn main() {
    let s1: String = read_string();
    let s2: String = read_string();

    let p1: usize = s1.chars().filter(|&x| x == '+').count();
    let m1: usize = s1.len() - p1;

    let p2: usize = s2.chars().filter(|&x| x == '+').count();
    let m2: usize = s2.chars().filter(|&x| x == '-').count();
    let r2: usize = s2.len() - p2 - m2;

    let p: i64 = (p1 as i64) - (p2 as i64);
    let m: i64 = (m1 as i64) - (m2 as i64);

    let fact: Vec<f64> = (0..=s1.len())
        .scan(1.0, |num, i| {
            *num *= 1.0f64.max(i as f64);
            Some(*num)
        })
        .collect();

    let res: f64 = if p < 0 || m < 0 || (r2 as i64) < p + m {
        0.0f64
    } else if p == 0 && m == 0 {
        1.0f64
    } else {
        fact[r2] / (fact[p as usize] * fact[m as usize] * 2.0f64.powf(r2 as f64))
    };

    let mut out = BufWriter::new(stdout().lock());
    writeln!(out, "{:.15}", res).unwrap();
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
