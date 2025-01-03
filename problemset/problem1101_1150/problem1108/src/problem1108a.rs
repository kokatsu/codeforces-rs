use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let q: usize = read();

    for _ in 0..q {
        let (l1, _r1, l2, r2): (u32, u32, u32, u32) = {
            let input: Vec<u32> = read_vec();
            (input[0], input[1], input[2], input[3])
        };

        let res = if l1 == l2 {
            format!("{} {}", l1, r2)
        } else {
            format!("{} {}", l1, l2)
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