use std::io::{stdout, Write, BufWriter};

fn main() {
    let t: usize = read();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let n: usize = read();
        let a: Vec<i64> = read_vec();

        let mut dp: Vec<i64> = vec![0; n+1];
        for (i, x) in a.iter().enumerate() {
            if (i as i64) + x >= (n as i64) {
                dp[n] = dp[n].max(dp[i]+x);
            }
            else {
                dp[i+(*x as usize)] = dp[i+(*x as usize)].max(dp[i]+x);
            }
        }

        let res: i64 = dp[n];

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