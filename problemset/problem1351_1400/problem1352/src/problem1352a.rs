use std::io::{stdout, BufWriter, Write};

fn main() {
    let t: usize = read();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let n: String = read_string();

        let numbers: Vec<usize> = n
            .chars()
            .rev()
            .enumerate()
            .map(|(i, c)| (c.to_digit(10).unwrap() as usize) * 10usize.pow(i.try_into().unwrap()))
            .filter(|&x| x > 0)
            .collect();

        writeln!(out, "{}", numbers.len()).unwrap();
        writeln!(
            out,
            "{}",
            numbers
                .iter()
                .fold(String::new(), |res, x| res + &x.to_string() + " ")
        )
        .unwrap();
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
