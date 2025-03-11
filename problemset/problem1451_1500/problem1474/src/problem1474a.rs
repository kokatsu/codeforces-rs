use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let n: usize = read();
        let b: String = read_string();

        let v: Vec<u32> = b.chars().map(|c| c.to_digit(10).unwrap()).collect();

        let mut a: Vec<u32> = vec![1; n];
        for i in 1..n {
            a[i] = if v[i] + 1 == a[i - 1] + v[i - 1] {
                0
            } else {
                1
            };
        }

        let res: String = a
            .iter()
            .map(|&x| x.to_string())
            .collect::<Vec<String>>()
            .join("");

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
