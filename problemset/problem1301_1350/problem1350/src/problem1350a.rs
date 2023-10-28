use std::io::{stdout, Write, BufWriter};

fn main() {
    let t: usize = read();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let input: Vec<u64> = read_vec();
        let n: u64 = input[0];
        let k: u64 = input[1];

        let res: u64 =
            if n % 2 == 0 {
                n + k * 2
            }
            else {
                let mut d: u64 = 3;
                while d * d <= n {
                    if n % d == 0 {
                        break;
                    }
                    d += 2;
                }
                if n % d != 0 {
                    d = n;
                }
                n + d + (k - 1) * 2
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