use std::io::{stdout, BufWriter, Write};

fn split_alpha_or_numeric(x: &[char]) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let mut is_alpha = x[0].is_alphabetic();

    for &c in x {
        if c.is_alphabetic() != is_alpha {
            result.push(String::new());
            is_alpha = !is_alpha;
        }

        if let Some(last) = result.last_mut() {
            last.push(c);
        } else {
            result.push(c.to_string());
        }
    }

    result
}

fn alpha_to_int(s: &str) -> u32 {
    let mut result = 0;

    for c in s.chars() {
        result = result * 26 + (c as u32 - 'A' as u32 + 1);
    }

    result
}

fn int_to_alpha(n: u32) -> String {
    let mut result = String::new();
    let mut num = n;

    while num > 0 {
        let rem = (num - 1) % 26;
        result.push((rem as u8 + b'A') as char);
        num = (num - 1) / 26;
    }

    result.chars().rev().collect()
}

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let s: Vec<char> = read_string().chars().collect();

        let a: Vec<String> = split_alpha_or_numeric(&s);

        let res: String = if a.len() == 2 {
            format!("R{}C{}", a[1], alpha_to_int(&a[0]))
        } else {
            format!("{}{}", int_to_alpha(a[3].parse::<u32>().unwrap()), a[1])
        };

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
