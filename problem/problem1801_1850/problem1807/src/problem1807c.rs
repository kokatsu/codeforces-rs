use std::io::{stdout, BufWriter, Write};

fn main() {
    let t: usize = read();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let _n: usize = read();
        let s: String = read_string();

        let (even, odd): (Vec<i64>, Vec<i64>) = s.chars().enumerate().fold(
            (vec![0; 26], vec![0; 26]),
            |(mut even, mut odd), (i, x)| {
                if i % 2 == 0 {
                    even[((x as u8) - ('a' as u8)) as usize] += 1;
                } else {
                    odd[((x as u8) - ('a' as u8)) as usize] += 1;
                }
                (even, odd)
            },
        );

        let num: usize = even
            .iter()
            .zip(odd.iter())
            .filter(|(x, y)| x > &&0 && y > &&0)
            .count();

        let res: &str = if num == 0 { "YES" } else { "NO" };

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
