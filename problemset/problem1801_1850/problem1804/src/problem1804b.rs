use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let (_n, k, d, w): (usize, i64, i64, i64) = {
            let input: Vec<i64> = read_vec();
            (input[0] as usize, input[1], input[2], input[3])
        };

        let a: Vec<i64> = read_vec();

        let (mut l, mut r): (i64, i64) = (-1, 0);
        let mut res: i64 = 0;
        for &x in a.iter() {
            if l < x || r <= 0 {
                res += 1;
                l = x + w + d;
                r = k - 1;
            } else {
                r -= 1;
            }
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
