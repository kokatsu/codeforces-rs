use std::io::{stdout, BufWriter, Write};

fn convert_char(n: u32, x: u32, y: u32) -> char {
    let u: u32 = n - x.abs_diff(n);
    let v: u32 = n - y.abs_diff(n);
    if u + v < n {
        return ' ';
    }
    char::from_u32(u + v - n + ('0' as u32)).unwrap()
}

fn main() {
    let t: usize = read();

    let mut out = BufWriter::new(stdout().lock());

    let n: usize = t * 2;
    for i in 0..=n {
        let a: Vec<char> = (0..=n)
            .map(|j| convert_char(t as u32, i as u32, j as u32))
            .collect();

        let line: String = (1..=n).fold(a[0].to_string(), |line, i| line + " " + &a[i].to_string());

        writeln!(out, "{}", line.trim_end()).unwrap();
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
