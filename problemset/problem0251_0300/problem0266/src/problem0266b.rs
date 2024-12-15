use std::io::{stdout, BufWriter, Write};

fn main() {
    let input: Vec<usize> = read_vec();
    let n: usize = input[0];
    let t: usize = input[1];

    let mut s: Vec<char> = read_string().chars().collect();

    for _ in 0..t {
        let mut swapped: bool = false;

        for i in (1..n).rev() {
            if swapped {
                swapped = false;
            } else if s[i - 1] == 'B' && s[i] == 'G' {
                s.swap(i - 1, i);
                swapped = true;
            }
        }
    }

    let res: String = s.iter().collect();

    let mut out = BufWriter::new(stdout().lock());
    writeln!(out, "{}", &res).unwrap();
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
