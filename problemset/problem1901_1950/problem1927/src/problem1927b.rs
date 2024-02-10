use std::io::{stdout, Write, BufWriter};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let n: usize = read();
        let a: Vec<usize> = read_vec();

        let mut letters: Vec<Vec<char>> = vec![Vec::<char>::new(); n+1];
        letters[0] = ('a'..='z')
            .map(|c| c as char)
            .collect::<Vec<char>>();

        let res: String = a
            .iter()
            .fold(String::new(), |res, x| {
                let c: char = letters[*x].pop().unwrap();
                letters[x+1].push(c);
                res + &c.to_string()
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