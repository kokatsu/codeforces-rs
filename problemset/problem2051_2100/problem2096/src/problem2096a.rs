use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let n: usize = read();
        let s: Vec<char> = read_string().chars().collect();

        let (mut l, mut r): (usize, usize) = (1, n);
        let mut a: Vec<usize> = vec![0; n];
        for i in (0..n - 1).rev() {
            if s[i] == '<' {
                a[i + 1] = l;
                l += 1;
            } else {
                a[i + 1] = r;
                r -= 1;
            }
        }

        a[0] = l;

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
