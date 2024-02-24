use std::io::{stdout, Write, BufWriter};
use std::collections::HashMap;

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let n: usize = read();

    let mut names: Vec<String> = vec![String::new(); n];
    let mut scores: Vec<i64> = vec![0; n];
    for i in 0..n {
        let input: Vec<String> = read_vec();
        names[i] = input[0].clone();
        scores[i] = input[1].parse().ok().unwrap();
    }

    let map1: HashMap<String, i64> = (0..n)
        .fold(HashMap::new(), |mut map1, i| {
            *map1.entry(names[i].clone()).or_insert(0) += scores[i];
            map1
        });

    let max_score: i64 = *map1.iter().map(|(_, score)| score).max().unwrap();
    let map2: HashMap<String, i64> = map1.into_iter().filter(|(_, score)| score == &max_score).collect();

    let mut map3: HashMap<String, i64> = HashMap::new();
    for i in 0..n {
        *map3.entry(names[i].clone()).or_insert(0) += scores[i];
        let score: i64 = *map3.get(&names[i]).unwrap();
        if score >= max_score && map2.contains_key(&names[i]) {
            writeln!(out, "{}", names[i]).unwrap();
            return;
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