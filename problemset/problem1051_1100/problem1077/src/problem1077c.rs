use std::collections::HashMap;
use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let _n: usize = read();
    let a: Vec<u64> = read_vec();

    let s: u64 = a.iter().sum();

    let mut map: HashMap<u64, i32> = HashMap::new();
    for &x in a.iter() {
        *map.entry(x).or_insert(0) += 1;
    }

    let mut numbers: Vec<usize> = Vec::new();
    for (i, &x) in a.iter().enumerate() {
        let m: u64 = s - x;
        if m % 2 == 0 {
            let d: u64 = m / 2;
            *map.entry(x).or_insert(0) -= 1;
            if map.get(&d).unwrap_or(&0) > &0 {
                numbers.push(i + 1);
            }
            *map.entry(x).or_insert(0) += 1;
        }
    }

    let l: usize = numbers.len();
    let t: String = numbers
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" ");

    let res: String = format!("{}\n{}", l, t);

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
