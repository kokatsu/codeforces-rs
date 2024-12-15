use std::io::{stdout, Write, BufWriter};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let n: usize = read();
        let a: Vec<u64> = read_vec();

        let mut alice: u64 = 0;
        let mut bob: u64 = 0;
        let mut rem: u64 = a.clone().iter().sum();
        let mut pre: u64 = 0;

        let mut l: usize = 0;
        let mut r: usize = n - 1;
        let mut count: usize = 0;
        while l <= r {
            count += 1;
            let is_alice: bool = count % 2 == 1;
            if rem <= pre {
                if is_alice {
                    alice += rem;
                }
                else {
                    bob += rem;
                }
                break;
            }

            let mut num: u64 = 0;
            while num <= pre {
                if is_alice {
                    alice += a[l];
                    num += a[l];
                    rem -= a[l];
                    l += 1;
                }
                else {
                    bob += a[r];
                    num += a[r];
                    rem -= a[r];
                    if r > 0 {
                        r -= 1;
                    }
                    else {
                        num = u64::MAX;
                        break;
                    }
                }
            }

            pre = num;
        }

        let res: String = format!("{} {} {}", count, alice, bob);

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