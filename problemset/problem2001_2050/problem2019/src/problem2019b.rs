use std::collections::HashMap;
use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let (n, q): (u64, usize) = {
            let input: Vec<u64> = read_vec();
            (input[0], input[1] as usize)
        };

        let x: Vec<u64> = read_vec();
        let k: Vec<u64> = read_vec();

        let mut map: HashMap<u64, u64> = HashMap::new();
        for (i, &v) in x.iter().enumerate() {
            let j: u64 = i as u64;
            let a: u64 = (j + 1) * (n - j) - 1;
            *map.entry(a).or_insert(0) += 1;
            let b: u64 = j * (n - j);
            *map.entry(b).or_insert(0) += v - if i > 0 { x[i - 1] + 1 } else { 1 };
        }

        let mut v: Vec<u64> = vec![0; q];
        for (i, &val) in k.iter().enumerate() {
            v[i] = *map.get(&val).unwrap_or(&0);
        }

        let res: String = v
            .iter()
            .map(|&val| val.to_string())
            .collect::<Vec<String>>()
            .join(" ");

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
