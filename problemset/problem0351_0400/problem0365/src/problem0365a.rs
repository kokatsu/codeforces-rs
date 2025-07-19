use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let (n, k): (u32, u32) = {
        let input: Vec<u32> = read_vec();
        (input[0], input[1])
    };

    let t: u32 = 2u32.pow(k + 1) - 1;

    let res: u8 = (0..n).fold(0, |res, _| {
        let mut a: u32 = read();

        let mut b: u32 = 0;
        while a > 0 {
            b |= 1 << (a % 10);
            a /= 10;
        }

        if b & t == t {
            res + 1
        } else {
            res
        }
    });

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
