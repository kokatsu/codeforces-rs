use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let (n, h, m): (usize, u32, usize) = {
        let input: Vec<usize> = read_vec();
        (input[0], input[1] as u32, input[2])
    };

    let mut a: Vec<u32> = vec![h; n];
    for _ in 0..m {
        let (l, r, x): (usize, usize, u32) = {
            let input: Vec<usize> = read_vec();
            (input[0], input[1], input[2] as u32)
        };

        for y in a.iter_mut().take(r).skip(l - 1) {
            *y = (*y).min(x);
        }
    }

    let res: u32 = a.iter().map(|&x| x * x).sum();

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
