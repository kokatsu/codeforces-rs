use std::io::{stdout, BufWriter, Write};

fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u')
}

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let s: Vec<char> = read_string().chars().collect();
    let t: Vec<char> = read_string().chars().collect();

    let res: &str = if s.len() != t.len() {
        "No"
    } else if s
        .iter()
        .zip(t.iter())
        .all(|(sc, tc)| is_vowel(*sc) == is_vowel(*tc))
    {
        "Yes"
    } else {
        "No"
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
