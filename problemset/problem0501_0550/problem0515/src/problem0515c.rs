use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let _n: usize = read();
    let mut a: u64 = read();

    let mut s: Vec<char> = Vec::new();
    while a > 0 {
        let d: usize = (a % 10) as usize;
        s.extend({
            match d {
                2 | 3 | 5 | 7 => vec![char::from_digit(d as u32, 10).unwrap()],
                4 => vec!['2', '2', '3'],
                6 => vec!['3', '5'],
                8 => vec!['2', '2', '2', '7'],
                9 => vec!['2', '3', '3', '7'],
                _ => vec![],
            }
        });
        a /= 10;
    }

    s.sort_by(|x, y| x.cmp(y).reverse());

    let res: String = s.iter().collect();

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
