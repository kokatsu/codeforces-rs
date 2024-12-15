use std::collections::HashMap;
use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let s: Vec<char> = read_string().chars().collect();

        let mut map: HashMap<char, usize> = s.iter().fold(HashMap::new(), |mut map, &c| {
            *map.entry(c).or_insert(0) += 1;
            map
        });

        let mut index: usize = 0;
        for x in s.iter() {
            let count: &mut usize = map.entry(*x).or_insert(1);
            *count -= 1;
            if *count == 0 {
                break;
            }
            index += 1;
        }

        let res: String = s[index..s.len()].iter().collect();

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
