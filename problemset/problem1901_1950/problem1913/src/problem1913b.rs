use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let s: Vec<char> = read_string().chars().collect();

        let mut zeros: usize = s.iter().filter(|&&c| c == '0').count();
        let mut ones: usize = s.len() - zeros;

        for &c in s.iter() {
            if c == '0' && ones > 0 {
                ones -= 1;
            } else if c == '1' && zeros > 0 {
                zeros -= 1;
            } else {
                break;
            }
        }

        let res: usize = zeros + ones;

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
