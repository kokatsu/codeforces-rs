use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let n: u32 = read();

    let mut a: u32 = 1;
    let mut b: u32 = n;
    let mut i: u32 = 1;
    while i * i <= n {
        if n.is_multiple_of(i) {
            let j: u32 = n / i;
            if j - i < b - a {
                a = i;
                b = j;
            }
        }
        i += 1;
    }

    let res: String = format!("{} {}", a, b);

    writeln!(out, "{}", res).unwrap();
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
