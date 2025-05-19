use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let n: u16 = read();

        let v: Vec<u16> = (0..n).fold(Vec::<u16>::new(), |mut acc, i| {
            let g: Vec<u16> = read_vec();
            if i < n - 1 {
                acc.push(g[0]);
            } else {
                acc.extend(g);
            }
            acc
        });

        let s: u16 = v.iter().sum();
        let m: u16 = (n * 2 + 1) * n - s;

        let res: String = format!(
            "{} {}",
            m,
            v.iter()
                .map(|&x| x.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        );

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
