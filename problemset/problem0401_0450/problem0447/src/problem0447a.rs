use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let (p, n): (usize, usize) = {
        let input: Vec<usize> = read_vec();
        (input[0], input[1])
    };

    let mut x: Vec<i32> = vec![0; n];
    for v in x.iter_mut().take(n) {
        *v = read();
    }

    let mut h: Vec<bool> = vec![false; p];
    let mut res: i32 = -1;
    for (i, &v) in x.iter().enumerate() {
        let m: usize = (v as usize) % p;
        if h[m] {
            res = i as i32 + 1;
            break;
        } else {
            h[m] = true;
        }
    }

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
