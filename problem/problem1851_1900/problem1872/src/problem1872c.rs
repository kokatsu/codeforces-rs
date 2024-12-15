use std::io::{stdout, BufWriter, Write};

fn solve(x: u64, y: u64) -> String {
    if y <= 3 {
        return "-1".to_string();
    }

    if y - x >= 1 {
        return format!("{} {}", y / 2, y / 2);
    }

    let s: u64 = (y as f64).sqrt().floor() as u64;
    for i in 2..=s {
        if y % i == 0 {
            return format!("{} {}", i, y - i);
        }
    }

    "-1".to_string()
}

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let (a, b): (u64, u64) = {
            let input: Vec<u64> = read_vec();
            (input[0], input[1])
        };

        let res: String = solve(a, b);

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
