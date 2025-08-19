use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let (mut n, k): (u32, usize) = {
        let v: Vec<u32> = read_vec();
        (v[0], v[1] as usize)
    };

    let mut l: Vec<u32> = Vec::new();
    let mut d: u32 = 2;
    while l.len() + 1 < k && n > 1 && d <= n {
        if n % d == 0 {
            l.push(d);
            n /= d;
        } else {
            d += 1;
        }
    }

    if n > 1 {
        l.push(n);
    }

    let res: String = if l.len() == k {
        l.iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    } else {
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
