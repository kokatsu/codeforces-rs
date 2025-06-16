use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let n: usize = read();

        let mut a: Vec<Vec<u16>> = Vec::with_capacity(n);
        for _ in 0..n {
            let s: String = read_string();
            a.push(s.chars().map(|c| c.to_digit(10).unwrap() as u16).collect());
        }

        let res: u16 = (0..n)
            .map(|i| {
                (i..n - i - 1)
                    .map(|j| {
                        let s: u16 =
                            a[i][j] + a[n - j - 1][i] + a[n - i - 1][n - j - 1] + a[j][n - i - 1];
                        s.min(4 - s)
                    })
                    .sum::<u16>()
            })
            .sum();

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
