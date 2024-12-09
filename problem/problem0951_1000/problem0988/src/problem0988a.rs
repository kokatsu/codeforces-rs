use std::collections::HashSet;
use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let (_n, k): (usize, usize) = {
        let input: Vec<usize> = read_vec();
        (input[0], input[1])
    };

    let a: Vec<u32> = read_vec();

    let mut set: HashSet<u32> = a.iter().cloned().collect();

    if set.len() >= k {
        writeln!(out, "YES").unwrap();
    } else {
        writeln!(out, "NO").unwrap();
        return;
    }

    let mut v: Vec<usize> = Vec::new();
    for (i, x) in a.iter().enumerate() {
        if set.contains(x) {
            v.push(i + 1);
            set.remove(x);
        }

        if v.len() >= k {
            break;
        }
    }

    let res: String = v
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" ");

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
