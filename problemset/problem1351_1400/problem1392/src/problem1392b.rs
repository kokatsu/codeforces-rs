use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let (_n, k): (usize, u64) = {
            let input: Vec<u64> = read_vec();
            (input[0] as usize, input[1])
        };

        let mut a: Vec<i64> = read_vec();

        let max: i64 = *a.iter().max().unwrap();
        for x in a.iter_mut() {
            *x = max - *x;
        }

        if k.is_multiple_of(2) {
            let max: i64 = *a.iter().max().unwrap();
            for x in a.iter_mut() {
                *x = max - *x;
            }
        }

        let res: String = a
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ");

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
