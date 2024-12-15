use std::io::{stdout, Write, BufWriter};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let res: String = solve();

        writeln!(out, "{}", res).unwrap();
    }
}

fn solve() -> String {
    let s: String = read_string();

    let n: i64 = s.clone().parse::<i64>().ok().unwrap();

    let mut m: i64 = 10;
    while n / m > 0 {
        let a: i64 = n / m;
        let b: i64 = n % m;

        m *= 10;

        if a >= b {
            continue;
        }

        let u: String = a.to_string();
        let v: String = b.to_string();
        let w: String = u.clone() + &v.clone();

        if w == s {
            return format!("{} {}", u, v);
        }
    }

    "-1".to_string()
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