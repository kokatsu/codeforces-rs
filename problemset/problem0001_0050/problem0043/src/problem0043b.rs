use std::collections::HashMap;
use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let s1: String = read();
    let s2: String = read();

    let mut char_count: HashMap<char, i32> = HashMap::new();
    for c in s1.chars().filter(|&c| c != ' ') {
        *char_count.entry(c).or_insert(0) += 1;
    }

    let ok: bool = s2.chars().filter(|&c| c != ' ').all(|c| {
        if let Some(count) = char_count.get_mut(&c) {
            if *count > 0 {
                *count -= 1;
                return true;
            }
        }
        false
    });

    let res: &str = if ok { "YES" } else { "NO" };

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
