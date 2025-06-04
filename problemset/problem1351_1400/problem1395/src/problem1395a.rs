use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let (r, g, b, w): (u64, u64, u64, u64) = {
            let input: Vec<u64> = read_vec();
            (input[0], input[1], input[2], input[3])
        };

        let odds: u64 = [r, g, b, w].iter().filter(|&&x| x % 2 == 1).count() as u64;

        let res: &str = if odds != 2 && !(odds >= 3 && [r, g, b].iter().any(|&x| x == 0)) {
            "Yes"
        } else {
            "No"
        };

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
