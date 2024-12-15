use std::io::{stdout, Write, BufWriter};
use std::collections::HashMap;

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let (n, k): (usize, usize) = {
            let input: Vec<usize> = read_vec();
            (input[0], input[1])
        };

        let map: HashMap<u32, u32> = (0..k)
            .fold(HashMap::new(), |mut map, _| {
                let (b, c): (u32, u32) = {
                    let input: Vec<u32> = read_vec();
                    (input[0], input[1])
                };
                *map.entry(b).or_insert(0) += c;
                map
            });

        let mut vec: Vec<u32> = map.into_values().collect();

        vec.sort_by(|x, y| x.cmp(y).reverse());

        let l: usize = vec.len().min(n).min(k);

        let res: u32 = vec[0..l].iter().sum();

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