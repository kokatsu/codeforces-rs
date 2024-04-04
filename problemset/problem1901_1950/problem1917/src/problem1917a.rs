use std::io::{stdout, Write, BufWriter};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let n: usize = read();
        let a: Vec<i64> = read_vec();

        let f: i64 = a
            .iter()
            .fold(1, |f, &x| {
                match x {
                    0 => 0,
                    x if x > 0 => f,
                    _ => -f,
                }
            });

        let res: String =
            if f > 0 {
                let z: usize = (0..n)
                    .filter(|&i| a[i] != 0)
                    .next()
                    .unwrap() + 1;
                1.to_string() + "\n" + &z.to_string() + " " + &0.to_string()
            }
            else {
                0.to_string()
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