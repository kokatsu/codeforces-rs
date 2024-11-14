use std::io::{stdout, Write, BufWriter};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let (_n, m): (usize, usize) = {
            let input: Vec<usize> = read_vec();
            (input[0], input[1])
        };

        let a: Vec<i64> = read_vec();

        let mut num: i64 = *a.iter().max().unwrap();
        let mut vec: Vec<i64> = Vec::new();
        for _ in 0..m {
            let (c, l, r): (String, i64, i64) = {
                let input: Vec<String> = read_vec();
                let l: i64 = input[1].parse().ok().unwrap();
                let r: i64 = input[2].parse().ok().unwrap();
                (input[0].clone(), l, r)
            };

            if l <= num && num <= r {
                num += if c == "+" { 1 } else { -1 };
            }

            vec.push(num);
        }

        let res: String = vec
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