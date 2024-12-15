use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let (_n, k): (usize, u32) = {
            let input: Vec<usize> = read_vec();
            (input[0], input[1] as u32)
        };

        let a: Vec<u32> = read_vec();

        let res: u32 = a
            .iter()
            .fold((0, 0), |(res, gold), &x| {
                if x >= k {
                    (res, gold + x)
                } else if x == 0 && gold > 0 {
                    (res + 1, gold - 1)
                } else {
                    (res, gold)
                }
            })
            .0;

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
