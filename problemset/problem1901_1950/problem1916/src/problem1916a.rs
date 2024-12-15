use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let input: Vec<usize> = read_vec();
        let (_n, k): (usize, usize) = (input[0], input[1]);

        let b: Vec<u64> = read_vec();

        let (rem, is_ok): (u64, bool) = b.iter().fold((2023, true), |(rem, is_ok), x| {
            if rem % x == 0 {
                (rem / x, is_ok)
            } else {
                (1, false)
            }
        });

        let res: String = if is_ok {
            (0..k - 1).fold("YES\n".to_string() + &rem.to_string(), |s, _| {
                s + " " + &1.to_string()
            })
        } else {
            "NO".to_string()
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
