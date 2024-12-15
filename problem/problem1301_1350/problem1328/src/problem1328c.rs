use std::io::{stdout, Write, BufWriter};

fn main() {
    let t: usize = read();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let _n: usize = read();
        let x: String = read_string();

        let mut seen1: bool = false;
        let mut a: String = String::new();
        let mut b: String = String::new();
        for c in x.chars() {
            if seen1 {
                a += "0";
                b += &c.to_string();
            }
            else {
                if c == '0' {
                    a += "0";
                    b += "0";
                }
                else if c == '1' {
                    seen1 = true;
                    a += "1";
                    b += "0";
                }
                else {
                    a += "1";
                    b += "1";
                }
            }
        }

        writeln!(out, "{}", &a).unwrap();
        writeln!(out, "{}", &b).unwrap();
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