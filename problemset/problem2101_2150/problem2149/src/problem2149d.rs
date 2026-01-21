use std::io::{stdout, BufWriter, Write};

fn calc(x: &[usize]) -> isize {
    if x.is_empty() {
        return 0;
    }

    let mid: usize = x.len() / 2;
    let mut ret: isize = 0;
    for (i, &v) in x.iter().enumerate() {
        let u: isize = x[mid] as isize - v as isize;
        let v: isize = mid as isize - i as isize;

        ret += (u - v).abs();
    }

    ret
}

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let n: usize = read();
        let s: Vec<char> = read_string().chars().collect();

        let a: Vec<usize> = (0..n).filter(|&i| s[i] == 'a').collect();
        let b: Vec<usize> = (0..n).filter(|&i| s[i] == 'b').collect();

        let res: isize = calc(&a).min(calc(&b));

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
