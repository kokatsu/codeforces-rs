use std::collections::HashSet;
use std::io::{stdout, BufWriter, Write};

fn main() {
    let t: usize = read();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let _input: Vec<usize> = read_vec();
        let a: Vec<u32> = read_vec();
        let b: HashSet<u32> = read_vec::<u32>().iter().cloned().collect();

        let mut exist: bool = false;
        for x in a.iter() {
            if b.contains(&x) && !exist {
                writeln!(out, "YES").unwrap();
                writeln!(out, "1 {}", x).unwrap();
                exist = true;
            }
        }

        if !exist {
            writeln!(out, "NO").unwrap();
        }
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
