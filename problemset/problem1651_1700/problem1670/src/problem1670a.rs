use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let n: usize = read();
        let mut a: Vec<i32> = read_vec();

        let mut neg: usize = a.iter().filter(|&&x| x < 0).count();
        for i in 0..n {
            if neg > 0 {
                if a[i] > 0 {
                    a[i] *= -1;
                }
                neg -= 1;
            } else if neg == 0 && a[i] < 0 {
                a[i] *= -1;
            }
        }

        let mut b: Vec<i32> = a.clone();
        b.sort();

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
