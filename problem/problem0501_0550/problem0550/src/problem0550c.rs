use std::io::{stdout, Write, BufWriter};
use std::iter::Peekable;
use std::str::Chars;

fn is_substring(x: &str, y: &str) -> bool {
    let binding = y.to_string();
    let mut v: Peekable<Chars<'_>> = binding.chars().peekable();

    for u in x.chars() {
        while let Some(&c) = v.peek() {
            if c == u {
                break;
            }
            v.next();
        }

        if v.peek().is_none() {
            return false;
        }
        v.next();
    }

    true
}

fn main() {
    let n: String = read_string();

    let mut out = BufWriter::new(stdout().lock());

    for i in (0..1000).step_by(8) {
        if is_substring(&i.to_string(), &n) {
            writeln!(out, "YES\n{}", i).unwrap();
            return;
        }
    }

    writeln!(out, "NO").unwrap();
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