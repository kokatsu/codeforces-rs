use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let (_n, k): (usize, u16) = {
        let input: Vec<usize> = read_vec();
        (input[0], input[1] as u16)
    };

    let a: Vec<u16> = read_vec();

    let mut res: i16 = -1;
    let mut s: u16 = 0;
    let mut t: u16 = 0;
    for (i, &x) in a.iter().enumerate() {
        s += x;
        let r: u16 = s.min(8);
        t += r;
        s -= r;

        if t >= k {
            res = (i + 1) as i16;
            break;
        }
    }

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
