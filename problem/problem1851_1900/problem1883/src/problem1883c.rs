use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let (_n, k): (usize, u8) = {
            let input: Vec<usize> = read_vec();
            (input[0], input[1] as u8)
        };

        let a: Vec<u8> = read_vec();

        let num: u8 = a
            .clone()
            .into_iter()
            .map(|x| if x % k == 0 { 0 } else { k - x % k })
            .min()
            .unwrap();

        let res: u8 = match k {
            4 => {
                let evens: Vec<u8> = a.into_iter().filter(|x| x % 2 == 0).collect();
                let f: u8 = if evens.iter().any(|x| x % 4 == 0) || evens.len() >= 2 {
                    0
                } else if !evens.is_empty() {
                    1
                } else {
                    2
                };
                f.min(num)
            }
            _ => num,
        };

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
