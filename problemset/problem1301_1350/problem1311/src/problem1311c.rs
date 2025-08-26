use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let (n, _m): (usize, usize) = {
            let input: Vec<usize> = read_vec();
            (input[0], input[1])
        };

        let s: Vec<char> = read_string().chars().collect();
        let p: Vec<usize> = read_vec();

        let mut c: Vec<Vec<usize>> = vec![vec![0; 26]; n + 1];
        for i in 1..=n {
            c[i] = c[i - 1].clone();
            c[i][s[i - 1] as usize - 'a' as usize] += 1;
        }

        let mut q: Vec<usize> = vec![0; 26];
        for &x in p.iter() {
            for (j, item) in q.iter_mut().enumerate().take(26) {
                *item += c[x][j];
            }
        }

        for (j, item) in q.iter_mut().enumerate().take(26) {
            *item += c[n][j];
        }

        let res: String = q
            .iter()
            .map(|&x| x.to_string())
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
