use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let (l, r): (i32, i32) = {
        let input: Vec<i32> = read_vec();
        (input[0], input[1])
    };

    let res: i32 = (l..=r)
        .find(|&x| {
            let mut digits: Vec<bool> = vec![false; 10];
            let mut n: usize = x as usize;
            while n > 0 {
                let d: usize = n % 10;
                if digits[d] {
                    return false;
                }
                digits[d] = true;
                n /= 10;
            }
            true
        })
        .unwrap_or(-1);

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
