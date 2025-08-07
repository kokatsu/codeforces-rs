use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let (n, m): (usize, usize) = {
        let input = read_vec::<usize>();
        (input[0], input[1])
    };

    let mut c: Vec<Vec<u16>> = vec![vec![0; m]; n];
    for row in c.iter_mut().take(n) {
        let input: Vec<u16> = read_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap_or(0) as u16)
            .collect();
        row[..m].copy_from_slice(&input[..m]);
    }

    let mut b: Vec<bool> = vec![false; n];
    for i in 0..m {
        let x: u16 = (0..n).map(|j| c[j][i]).max().unwrap_or(0);

        for j in 0..n {
            if c[j][i] == x {
                b[j] = true;
            }
        }
    }

    let res: usize = b.iter().filter(|&&x| x).count();

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
