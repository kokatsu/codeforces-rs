use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let a: usize = 'a' as usize;

    let input: Vec<usize> = read_vec();
    let (_n, _k): (usize, usize) = (input[0], input[1]);

    let s: Vec<usize> = read_string().chars().map(|x| x as usize - a).collect();

    let c: Vec<usize> = read_vec::<char>().iter().map(|&x| x as usize - a).collect();

    let mut can: Vec<bool> = vec![false; 26];
    for &x in c.iter() {
        can[x] = true;
    }

    let (num, len): (u64, u64) = s.iter().fold((0_u64, 0_u64), |(num, len), &x| {
        if can[x] {
            (num, len + 1)
        } else {
            (num + len * (len + 1) / 2, 0)
        }
    });

    let res: u64 = num + len * (len + 1) / 2;

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
