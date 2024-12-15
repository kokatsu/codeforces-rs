use std::io::{stdout, Write, BufWriter};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let (n, c, d): (usize, u64, u64) = {
            let input: Vec<u64> = read_vec();
            (input[0] as usize, input[1], input[2])
        };

        let mut b: Vec<u64> = read_vec();

        b.sort();

        let mut a: Vec<u64> = vec![0; n*n];
        a[0] = b[0];
        for i in 0..n {
            for j in 0..n {
                if i == 0 && j == 0 {
                    continue;
                }

                a[i*n+j] = a[0] + i as u64 * c + j as u64 * d;
            }
        }

        a.sort();

        let res: &str = if a == b { "YES" } else { "NO" };

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