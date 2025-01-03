use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let (n, m): (usize, usize) = {
        let input: Vec<usize> = read_vec();
        (input[0], input[1])
    };

    let mut imos: Vec<i8> = vec![0; m + 2];

    for _ in 0..n {
        let (l, r): (usize, usize) = {
            let input: Vec<usize> = read_vec();
            (input[0], input[1])
        };

        imos[l] += 1;
        imos[r + 1] -= 1;
    }

    let mut list: Vec<usize> = Vec::new();
    for i in 1..=m {
        imos[i] += imos[i - 1];
        if imos[i] == 0 {
            list.push(i);
        }
    }

    let s: String = list.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ");

    let res: String = list.len().to_string() + "\n" + &s;

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
