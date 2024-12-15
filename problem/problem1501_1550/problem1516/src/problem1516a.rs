use std::io::{stdout, Write, BufWriter};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let (n, mut k): (usize, usize) = {
            let input: Vec<usize> = read_vec();
            (input[0], input[1])
        };

        let mut a: Vec<usize> = read_vec();

        let res: String = (0..n)
            .fold(String::new(), |res, i| {
                let m: usize = k.min(a[i]);
                a[i] -= m;
                a[n-1] += m;
                k -= m;
                if res.is_empty() {
                    res + &a[i].to_string()
                }
                else {
                    res + " " + &a[i].to_string()
                }
            });

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