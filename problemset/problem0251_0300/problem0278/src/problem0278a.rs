use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let n: usize = read();
    let d: Vec<u32> = read_vec();
    let (s, t): (usize, usize) = {
        let mut input: Vec<usize> = read_vec();
        input.sort();
        (input[0], input[1])
    };

    let c: Vec<u32> = (0..n * 2 + 1)
        .scan(0, |acc, i| {
            if i > n {
                *acc += d[i - n - 1];
            } else if i > 0 {
                *acc += d[i - 1];
            }
            Some(*acc)
        })
        .collect();

    let res: u32 = (c[t - 1] - c[s - 1]).min(c[n + s - 1] - c[t - 1]);

    writeln!(out, "{}", res).unwrap();
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
