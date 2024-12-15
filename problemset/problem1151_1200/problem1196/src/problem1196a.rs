use std::io::{stdout, BufWriter, Write};

fn main() {
    let q: usize = read();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..q {
        let a: Vec<u64> = read_vec();

        let res: u64 = vec![(0, 1, 2), (0, 2, 1), (1, 2, 0)]
            .iter()
            .fold(0, |res, (i, j, k)| {
                let mut x: u64 = a[*i];
                let mut y: u64 = a[*j];
                if x < y {
                    std::mem::swap(&mut x, &mut y);
                }
                let z: u64 = a[*k];
                if x - y <= z {
                    res.max(x + (z + y - x) / 2)
                } else {
                    res.max(y + z)
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
