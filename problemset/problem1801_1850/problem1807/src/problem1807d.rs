use std::io::{stdout, BufWriter, Write};

fn main() {
    let t: usize = read();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let input: Vec<usize> = read_vec();
        let n: usize = input[0];
        let q: usize = input[1];

        let a: Vec<u64> = read_vec();

        let mut b: Vec<u64> = vec![0; n + 1];
        for i in 0..n {
            b[i + 1] = a[i];
        }

        let c: Vec<u64> = b
            .iter()
            .scan(0, |c, x| {
                *c += x % 2;
                Some(*c)
            })
            .collect();

        for _ in 0..q {
            let query: Vec<usize> = read_vec();
            let l: usize = query[0];
            let r: usize = query[1];
            let k: u64 = query[2] as u64 % 2;

            let x: u64 = (c[r] - c[l - 1]) % 2;
            let y: u64 = (k * (r - l + 1) as u64) % 2;

            let res: &str = if (c[n] - x + y) % 2 == 1 { "YES" } else { "NO" };

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
