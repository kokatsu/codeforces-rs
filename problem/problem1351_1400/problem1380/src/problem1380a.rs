use std::io::{stdout, BufWriter, Write};

fn solve(n: usize, p: &[usize]) -> String {
    for j in 1..n - 1 {
        if let Some(i) = (0..j).find(|&i| p[i] < p[j]) {
            if let Some(k) = (j + 1..n).find(|&k| p[j] > p[k]) {
                let three: String = [i, j, k]
                    .iter()
                    .map(|x| (x + 1).to_string())
                    .collect::<Vec<String>>()
                    .join(" ");
                return "YES\n".to_string() + &three;
            }
        }
    }

    "NO".to_string()
}

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let n: usize = read();
        let p: Vec<usize> = read_vec();

        let res: String = solve(n, &p);

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
