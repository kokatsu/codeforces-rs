use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let _n: usize = read();
        let a: Vec<i8> = read_vec();

        let min: i8 = *a.iter().min().unwrap_or(&i8::MIN);
        let max: i8 = *a.iter().max().unwrap_or(&i8::MAX);

        let res: String = if min >= 0 {
            let v: Vec<String> = (0..=max).map(|i| i.to_string()).collect();
            format!("YES\n{}\n{}", v.len(), v.join(" "))
        } else {
            "NO".to_string()
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
