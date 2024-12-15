use std::io::{stdout, BufWriter, Write};

fn solve(n: usize, k: usize, c: &[usize]) -> &str {
    let a: Vec<usize> = (0..n).filter(|&i| c[i] == c[0]).collect();

    if a.len() >= k {
        let b: Vec<usize> = (a[k - 1]..n).filter(|&i| c[i] == c[n - 1]).collect();

        if c[0] == c[n - 1] || b.len() >= k {
            return "YES";
        }
    }

    "NO"
}

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let (n, k): (usize, usize) = {
            let input: Vec<usize> = read_vec();
            (input[0], input[1])
        };

        let c: Vec<usize> = read_vec();

        let res: &str = solve(n, k, &c);

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
