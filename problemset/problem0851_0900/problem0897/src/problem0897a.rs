use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let (_n, m): (usize, usize) = {
        let input: Vec<usize> = read_vec();
        (input[0], input[1])
    };

    let mut s: Vec<char> = read_string().chars().collect();

    for _ in 0..m {
        let (l, r, c1, c2): (usize, usize, char, char) = {
            let input: Vec<String> = read_vec();
            (
                input[0].parse().ok().unwrap(),
                input[1].parse().ok().unwrap(),
                input[2].chars().next().unwrap(),
                input[3].chars().next().unwrap(),
            )
        };

        for x in s[(l - 1)..r].iter_mut() {
            if *x == c1 {
                *x = c2;
            }
        }
    }

    let res: String = s.into_iter().collect();

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
