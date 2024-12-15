use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let n: usize = read();
        let s: Vec<char> = read_string().chars().collect();

        let r: usize = (n as f64).sqrt().floor() as usize;

        let is_beautiful: bool = match r * r == n {
            true => {
                let top: bool = s[0..r].iter().all(|&c| c == '1');
                let bottom: bool = s[r * (r - 1)..n].iter().all(|&c| c == '1');
                let left: bool = s.iter().step_by(r).all(|&c| c == '1');
                let right: bool = s[r - 1..n].iter().step_by(r).all(|&c| c == '1');
                let inside: bool = s.iter().filter(|&&c| c == '1').count() == (r - 1) * 4;
                top && bottom && left && right && inside
            }
            _ => false,
        };

        let res: &str = if is_beautiful { "Yes" } else { "No" };

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
