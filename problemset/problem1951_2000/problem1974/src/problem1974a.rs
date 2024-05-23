use std::io::{stdout, Write, BufWriter};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let (mut x, mut y): (u32, u32) = {
            let input: Vec<u32> = read_vec();
            (input[0], input[1])
        };

        let mut res: u32 = 0;
        while x > 0 || y > 0 {
            let v: u32 = y.min(2);
            let u: u32 = x.min(15-v*4);

            x -= u;
            y -= v;
            res += 1;
        }

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