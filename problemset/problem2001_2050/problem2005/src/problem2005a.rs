use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let vowels: &str = "aeiou";

    let t: usize = read();

    for _ in 0..t {
        let n: usize = read();

        let res: String = vowels
            .chars()
            .enumerate()
            .map(|(i, x)| {
                let m: usize = n / 5 + if i + 1 <= n % 5 { 1 } else { 0 };
                x.to_string().repeat(m)
            })
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
