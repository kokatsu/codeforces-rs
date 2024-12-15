use std::io::{stdout, Write, BufWriter};

fn compare(x: String, y: String) -> String {
    if x == y {
        return "=".to_string();
    }

    let u: Vec<char> = x.chars().collect();
    let v: Vec<char> = y.chars().collect();

    let s: usize = u.len();
    let t: usize = v.len();

    if u[s-1] == v[t-1] {
        if u[s-1] == 'S' {
            if s < t {
                ">".to_string()
            }
            else {
                "<".to_string()
            }
        }
        else {
            if s < t {
                "<".to_string()
            }
            else {
                ">".to_string()
            }
        }
    }
    else {
        if (u[s-1] as u64) < (v[t-1] as u64) {
            ">".to_string()
        }
        else {
            "<".to_string()
        }
    }
}

fn main() {
    let t: usize = read();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let input: Vec<String> = read_vec();
        let a: String = input[0].clone();
        let b: String = input[1].clone();

        let res: String = compare(a, b);

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