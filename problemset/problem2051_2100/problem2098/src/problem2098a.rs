use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let s: Vec<usize> = read_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap_or_default() as usize)
            .collect();

        let mut c: Vec<usize> = vec![0; 10];
        for i in 0..s.len() {
            c[s[i]] += 1;
        }

        let res: String = (1..=10).fold(String::new(), |acc, i| {
            let j: usize = (10 - i..10).find(|&j| c[j] > 0).unwrap_or(0);
            if c[j] > 0 {
                c[j] -= 1;
                acc + &j.to_string()
            } else {
                acc
            }
        });

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
