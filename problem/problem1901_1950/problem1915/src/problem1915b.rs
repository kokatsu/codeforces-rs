use std::io::{stdout, Write, BufWriter};

fn main() {
    let t: usize = read();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let (a, b, _c): (usize, usize, usize) = (0..3)
            .fold((0, 0, 0), |(a, b, c), _| {
                let input: String = read_string();
                (a+get_char_count(&input, 'A'), b+get_char_count(&input, 'B'), c+get_char_count(&input, 'C'))
            });

        let res: char =
            if a == 2 {
                'A'
            }
            else if b == 2 {
                'B'
            }
            else {
                'C'
            };

        writeln!(out, "{}", res).unwrap();
    }
}

fn get_char_count(s: &str, c: char) -> usize {
    s.chars().filter(|&x| x == c).count()
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