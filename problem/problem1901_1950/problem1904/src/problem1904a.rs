use std::collections::HashSet;
use std::io::{stdout, BufWriter, Write};

fn kronecker(x: i32) -> i32 {
    match x % 8 {
        1 | 3 => 1,
        5 | 7 => -1,
        _ => 0,
    }
}

fn coord(a: i32, b: i32, x: i32, y: i32, z: i32) -> (i32, i32) {
    let u: i32 = x + a * kronecker(z + 1) + b * kronecker(z);
    let v: i32 = y + a * kronecker(z + 2) + b * kronecker(z + 3);
    (u, v)
}

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let (a, b): (i32, i32) = {
            let input: Vec<i32> = read_vec();
            (input[0], input[1])
        };

        let (xk, yk): (i32, i32) = {
            let input: Vec<i32> = read_vec();
            (input[0], input[1])
        };

        let (xq, yq): (i32, i32) = {
            let input: Vec<i32> = read_vec();
            (input[0], input[1])
        };

        let mut setk: HashSet<(i32, i32)> = HashSet::new();
        let mut setq: HashSet<(i32, i32)> = HashSet::new();
        for i in 0..8 {
            setk.insert(coord(a, b, xk, yk, i));
            setq.insert(coord(a, b, xq, yq, i));
        }

        let res: usize = setq.iter().filter(|&x| setk.contains(&x)).count();

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
