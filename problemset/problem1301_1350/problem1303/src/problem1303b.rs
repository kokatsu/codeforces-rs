use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let (n, g, b): (u64, u64, u64) = {
            let input: Vec<u64> = read_vec();
            (input[0], input[1], input[2])
        };

        let m: u64 = n.div_ceil(2);

        let d: u64 = m / g;
        let r: u64 = m % g;

        let s: u64 = d * (g + b);
        let t: u64 = if r == 0 { s - b } else { s + r };

        let res: u64 = t.max(n);

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
