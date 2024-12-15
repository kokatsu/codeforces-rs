use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let n: usize = read();
    let s: Vec<char> = read_string().chars().collect();

    let mut rl: usize = s.iter().filter(|&&x| x == 'L').count();
    let mut ro: usize = n - rl;

    let (mut ll, mut lo): (usize, usize) = (0, 0);
    let mut res: i32 = -1;
    for &c in s[0..n - 1].iter() {
        if c == 'L' {
            ll += 1;
            rl -= 1;
        } else {
            lo += 1;
            ro -= 1;
        }

        if ll != rl && lo != ro {
            res = (ll + lo) as i32;
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
