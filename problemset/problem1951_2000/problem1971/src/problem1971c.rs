use std::io::{stdout, Write, BufWriter};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let (a, b, c, d): (u8, u8, u8, u8) = {
            let mut input: Vec<u8> = read_vec();
            for i in 0..2 {
                if input[i*2] > input[i*2+1] {
                    input.swap(i * 2, i * 2 + 1);
                }
            }
            (input[0], input[1], input[2], input[3])
        };

        let res: &str =
            if (a < c && c < b && b < d) || (c < a && a < d && d < b) {
                "YES"
            }
            else {
                "NO"
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