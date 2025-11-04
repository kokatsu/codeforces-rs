use std::io::{stdout, BufWriter, Write};

const LIMIT: usize = 20_001;

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let mut sieve: Vec<bool> = vec![true; LIMIT];
    sieve[0] = false;
    sieve[1] = false;
    let mut d: usize = 2;
    while d * d < LIMIT {
        if sieve[d] {
            for i in (d * d..LIMIT).step_by(d) {
                sieve[i] = false;
            }
        }
        d += 1;
    }

    let t: usize = read();

    for _ in 0..t {
        let n: usize = read();
        let a: Vec<usize> = read_vec();

        let s: usize = a.iter().sum();

        let b: bool = !sieve[s];
        let index: Option<usize> = a.iter().position(|&x| !sieve[s - x]);

        let res: String = if b {
            format!(
                "{}\n{}",
                n,
                (1..=n)
                    .map(|i| i.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            )
        } else {
            format!(
                "{}\n{}",
                n - 1,
                (1..=n)
                    .filter(|&i| i - 1 != index.unwrap())
                    .map(|i| i.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            )
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
