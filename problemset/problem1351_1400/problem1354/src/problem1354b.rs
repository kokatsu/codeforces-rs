use std::io::{stdout, Write, BufWriter};

fn main() {
    let t: usize = read();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let s: String = read_string();

        let n: usize = s.len();

        let mut res: usize = n * 3;
        let mut pos: Vec<usize> = vec![n*3; 3];
        for (i, x) in s.chars().enumerate() {
            let p: usize = x as usize - '1' as usize;
            pos[p] = i;

            let min: usize = *pos.iter().min().unwrap();
            let max: usize = *pos.iter().max().unwrap();

            res = res.min(max-min+1);
        }

        if res > n {
            res = 0;
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