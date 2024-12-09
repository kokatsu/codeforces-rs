use std::io::{stdout, BufWriter, Write};

fn main() {
    let n: usize = read();
    let a: Vec<usize> = read_vec();

    let mut dp: Vec<Vec<usize>> = vec![vec![0; 3]; n + 1];
    for i in 0..n {
        dp[i + 1][0] = *dp[i].iter().max().unwrap();

        if a[i] & 1 == 1 {
            dp[i + 1][1] = dp[i][0].max(dp[i][2]) + 1;
        }

        if (a[i] >> 1) & 1 == 1 {
            dp[i + 1][2] = dp[i][0].max(dp[i][1]) + 1;
        }
    }

    let res: usize = n - *dp[n].iter().max().unwrap();

    let mut out = BufWriter::new(stdout().lock());
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
