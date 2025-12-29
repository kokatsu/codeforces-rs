use std::io::{stdout, BufWriter, Write};

fn get_vowels_count(chars: &[char]) -> usize {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    chars
        .iter()
        .filter(|c| vowels.contains(&c.to_ascii_lowercase()))
        .count()
}

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let a: Vec<char> = read_string().chars().collect();
    let b: Vec<char> = read_string().chars().collect();
    let c: Vec<char> = read_string().chars().collect();

    let x: usize = get_vowels_count(&a);
    let y: usize = get_vowels_count(&b);
    let z: usize = get_vowels_count(&c);

    let res = if x == 5 && y == 7 && z == 5 {
        "YES"
    } else {
        "NO"
    };

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
