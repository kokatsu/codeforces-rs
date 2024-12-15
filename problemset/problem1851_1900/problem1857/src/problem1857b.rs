use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let x: Vec<char> = read_string().chars().collect();

        let l: usize = x.len();
        let res: String = if let Some(i) = x.iter().position(|x| x.to_digit(10).unwrap() >= 5) {
            let mut j: usize = i;
            for &c in x[0..i].iter().rev() {
                if c != '4' {
                    break;
                }
                j -= 1;
            }
            let d: String = if j == 0 {
                "1".to_string()
            } else {
                let p: String = x[0..j - 1].iter().collect();
                p + &(x[j - 1].to_digit(10).unwrap() + 1).to_string()
            };
            d + &"0".to_string().repeat(l - j)
        } else {
            x.iter().collect()
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
