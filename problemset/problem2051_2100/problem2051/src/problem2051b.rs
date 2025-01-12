use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let (n, a, b, c): (u32, u32, u32, u32) = {
            let input: Vec<u32> = read_vec();
            (input[0], input[1], input[2], input[3])
        };

        let s: u32 = a + b + c;

        let res: u32 = n / s * 3
            + match n % s {
                0 => 0,
                x if x <= a => 1,
                x if x <= a + b => 2,
                _ => 3,
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
