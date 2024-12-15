use std::io::{stdout, BufWriter, Write};

fn main() {
    let t: usize = read();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let n: usize = read();
        let a: Vec<u64> = read_vec();

        let odd: Vec<u64> = a.iter().filter(|&x| x % 2 == 1).cloned().collect();

        let even: Vec<u64> = a.iter().filter(|&x| x % 2 == 0).cloned().collect();

        let l: usize = odd.len();

        let res: String = (0..n)
            .fold(String::new(), |res, i| {
                if i < l {
                    res + " " + &odd[i].to_string()
                } else {
                    res + " " + &even[i - l].to_string()
                }
            })
            .trim()
            .to_string();

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
