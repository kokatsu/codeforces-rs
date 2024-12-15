use std::io::{stdout, BufWriter, Write};

fn main() {
    let t: usize = read();

    let c: Vec<char> = "vika".chars().collect();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let input: Vec<usize> = read_vec();
        let n: usize = input[0];
        let m: usize = input[1];

        let carpet: Vec<Vec<char>> = (0..n)
            .map(|_| read_string().chars().collect::<Vec<char>>())
            .collect();

        let mut p: usize = 0;
        for j in 0..m {
            if p >= 4 {
                break;
            }

            for i in 0..n {
                if carpet[i][j] == c[p] {
                    p += 1;
                    break;
                }
            }
        }

        let res: &str = if p >= 4 { "YES" } else { "NO" };

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
