use std::io::{stdout, BufWriter, Write};

fn main() {
    let t: usize = read();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let n: usize = read();
        let mut a: Vec<i64> = read_vec();

        a.sort_by(|x, y| x.cmp(y).reverse());

        let diff = (0..n).fold(0, |diff, i| {
            if i % 2 == 0 && a[i] % 2 == 0 {
                diff + a[i]
            } else if i % 2 == 1 && a[i] % 2 == 1 {
                diff - a[i]
            } else {
                diff
            }
        });

        let res: &str = if diff > 0 {
            "Alice"
        } else if diff < 0 {
            "Bob"
        } else {
            "Tie"
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
