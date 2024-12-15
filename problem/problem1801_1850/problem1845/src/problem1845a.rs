use std::io::{stdout, BufWriter, Write};
use std::iter::repeat;

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let input: Vec<usize> = read_vec();
        let (mut n, k, x): (usize, usize, usize) = (input[0], input[1], input[2]);

        let a: Vec<usize> = if x == 1 {
            let mut b: Vec<usize> = Vec::new();
            if k >= 2 {
                b = repeat(2).take(n / 2).collect::<Vec<usize>>();
                n -= n / 2 * 2;
            }
            if n == 1 && k >= 3 {
                b[0] += 1;
                n -= 1;
            }
            b
        } else {
            let b: Vec<usize> = repeat(1).take(n).collect::<Vec<usize>>();
            n = 0;
            b
        };

        let res: String = if n == 0 {
            (1..a.len()).fold(
                "YES\n".to_string() + &a.len().to_string() + "\n" + &a[0].to_string(),
                |res, i| res + " " + &a[i].to_string(),
            )
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
