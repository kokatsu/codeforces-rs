use std::io::{stdout, Write, BufWriter};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let n: usize = read();
        let words: Vec<char> = read_string().chars().collect();

        let mut exists_dot: Vec<bool> = vec![false; n];
        let mut index: usize = n - 1;
        while index > 0 {
            if words[index] == 'a' || words[index] == 'e' {
                if index == 1 {
                    break;
                }

                exists_dot[index-1] = true;
                index -= 2;
            }
            else {
                if index == 2 {
                    break;
                }

                exists_dot[index-2] = true;
                index -= 3;
            }
        }

        let res: String = (0..n)
            .fold(String::new(), |res, i| {
                let dot: &str = if exists_dot[i] { "." } else { "" };
                res + dot + &words[i].to_string()
            });

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