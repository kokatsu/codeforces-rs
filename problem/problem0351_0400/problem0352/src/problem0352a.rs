use std::io::{stdout, Write, BufWriter};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let n: usize = read();
    let a: Vec<u8> = read_vec();

    let fives: usize = a.into_iter().filter(|&x| x == 5_u8).count();
    let zeros: usize = n - fives;

    let res: String =
        if zeros > 0 {
            let s: String = "5".to_string().repeat(fives/9*9);
            let zero: String = "0".to_string();
            if s.is_empty() {
                zero
            }
            else {
                s + &zero.repeat(zeros)
            }
        }
        else {
            "-1".to_string()
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