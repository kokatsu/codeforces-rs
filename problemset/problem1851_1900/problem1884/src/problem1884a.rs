use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let (x, k): (u32, u32) = {
            let input: Vec<u32> = read_vec();
            (input[0], input[1])
        };

        let res: u32 = (x..)
            .find(|i| {
                let s: &str = &i.to_string();
                let n: u32 = s.as_bytes().iter().map(|&b| (b - b'0') as u32).sum();
                n % k == 0
            })
            .unwrap_or(0);

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
