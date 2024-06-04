use std::io::{stdout, Write, BufWriter};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let (n, q): (u64, u64) = {
            let input: Vec<u64> = read_vec();
            (input[0], input[1])
        };

        let a: Vec<u64> = read_vec();

        let mut evens: u64 = a.clone().into_iter().filter(|x| x % 2 == 0).count() as u64;
        let mut odds: u64 = n - evens;

        let mut res: u64 = a.iter().sum();

        for _ in 0..q {
            let (typ, x): (u64, u64) = {
                let input: Vec<u64> = read_vec();
                (input[0], input[1])
            };

            if typ == 0 {
                res += x * evens;
                if x % 2 == 1 {
                    (evens, odds) = (0, n)
                }
            }
            else {
                res += x * odds;
                if x % 2 == 1 {
                    (evens, odds) = (n, 0)
                }
            }

            writeln!(out, "{}", res).unwrap();
        }
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