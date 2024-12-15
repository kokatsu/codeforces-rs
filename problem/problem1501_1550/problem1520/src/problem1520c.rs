use std::io::{stdout, Write, BufWriter};

fn main() {
    let t: usize = read();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let n: usize = read();

        if n == 2 {
            writeln!(out, "-1").unwrap();
            continue;
        }

        let mut num: i64 = 1;
        let mut res: Vec<Vec<i64>> = vec![vec![0; n]; n];
        for i in 0..=1 {
            for j in 0..n {
                for k in 0..n {
                    if (j + k) % 2 == i {
                        res[j][k] = num;
                        num += 1;
                    }
                }
            }
        }

        for i in 0..n {
            writeln!(out, "{}", res[i].iter().fold(String::new(), |x, &y| x + &y.to_string() + " ")).unwrap();
        }
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