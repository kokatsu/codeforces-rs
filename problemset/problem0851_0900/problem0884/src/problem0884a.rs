use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let (_n, t): (usize, u32) = {
        let input: Vec<usize> = read_vec();
        (input[0], input[1] as u32)
    };

    let a: Vec<u32> = read_vec();

    let mut res: usize = 0;
    let mut rem: u32 = t;
    for &x in a.iter() {
        if rem > 0 {
            res += 1;
            rem = rem.saturating_sub(86_400 - x);
        } else {
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
