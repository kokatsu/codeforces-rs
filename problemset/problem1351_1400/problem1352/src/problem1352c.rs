use std::io::{stdout, Write, BufWriter};

fn main() {
    let t: usize = read();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let input: Vec<i64> = read_vec();
        let n: i64 = input[0];
        let k: i64 = input[1];

        let mut ok: i64 = 0;
        let mut ng: i64 = std::i64::MAX / 2;

        let res: i64 =
            loop {
                if ng - ok <= 1 {
                    break ok + 1;
                }
                let mid: i64 = (ok + ng) / 2;
                if mid - mid / n < k {
                    ok = mid;
                }
                else {
                    ng = mid;
                }
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