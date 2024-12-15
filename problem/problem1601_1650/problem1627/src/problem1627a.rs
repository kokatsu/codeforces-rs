use std::io::{stdout, Write, BufWriter};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let (n, _m, r, c): (usize, usize, usize, usize) = {
            let input: Vec<usize> = read_vec();
            (input[0], input[1], input[2]-1, input[3]-1)
        };

        let (is_black, row_col_black, contain_black): (bool, bool, bool) = (0..n)
            .fold((false, false, false), |(is_black, row_col_black, contain_black), i| {
                let row: Vec<char> = read_string().chars().collect();
                match (i == r, row[c] == 'B', row.into_iter().any(|x| x == 'B')) {
                    (true, true, _) => (true, row_col_black, contain_black),
                    (true, false, true) | (false, true, _) => (is_black, true, contain_black),
                    (false, false, true) => (is_black, row_col_black, true),
                    _ => (is_black, row_col_black, contain_black),
                }
            });

        let res: i64 =
            if is_black {
                0
            }
            else if row_col_black {
                1
            }
            else if contain_black {
                2
            }
            else {
                -1
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