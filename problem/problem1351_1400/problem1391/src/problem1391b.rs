use std::io::{stdout, Write, BufWriter};

fn main() {
    let t: usize = read();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let input: Vec<usize> = read_vec();
        let n: usize = input[0];
        let m: usize = input[1];

        let res: usize =
            (0..n)
            .fold(0, |res, i| {
                let a: Vec<char> = read_string().chars().collect();
                if i < n - 1 {
                    if a[m-1] == 'R' {
                        res + 1
                    }
                    else {
                        res
                    }
                }
                else {
                    res + a[0..m-1].iter().filter(|&&c| c == 'D').count()
                }
            });

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