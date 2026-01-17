use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let (n, _m): (usize, usize) = {
            let input: Vec<usize> = read_vec();
            (input[0], input[1])
        };

        let field: Vec<Vec<char>> = (0..n).map(|_| read_string().chars().collect()).collect();

        let (mut u, mut v) = (0usize, 0usize);
        'outer: for (i, row) in field.iter().enumerate() {
            for (j, &c) in row.iter().enumerate() {
                if c == 'R' {
                    u = i;
                    v = j;
                    break 'outer;
                }
            }
        }

        let mut is_possible = true;
        'outer: for (i, row) in field.iter().enumerate().skip(u + 1) {
            for (j, &c) in row.iter().enumerate().take(v) {
                if c == 'R' && (i < u || j < v) {
                    is_possible = false;
                    break 'outer;
                }
            }
        }

        let res: &str = if is_possible { "YES" } else { "NO" };

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
