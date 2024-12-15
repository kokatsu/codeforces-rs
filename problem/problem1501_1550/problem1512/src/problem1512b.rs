use std::io::{stdout, BufWriter, Write};

fn main() {
    let t: usize = read();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let n: usize = read();

        let mut pos: Vec<(usize, usize)> = Vec::new();
        let mut table: Vec<Vec<char>> = vec![Vec::new(); n];
        for i in 0..n {
            table[i] = read_string().chars().collect();
            for j in 0..n {
                if table[i][j] == '*' {
                    pos.push((i, j));
                }
            }
        }

        if pos[0].0 == pos[1].0 {
            if pos[0].0 > 0 {
                table[pos[0].0 - 1][pos[0].1] = '*';
                table[pos[1].0 - 1][pos[1].1] = '*';
            } else {
                table[pos[0].0 + 1][pos[0].1] = '*';
                table[pos[1].0 + 1][pos[1].1] = '*';
            }
        } else if pos[0].1 == pos[1].1 {
            if pos[0].1 > 0 {
                table[pos[0].0][pos[0].1 - 1] = '*';
                table[pos[1].0][pos[1].1 - 1] = '*';
            } else {
                table[pos[0].0][pos[0].1 + 1] = '*';
                table[pos[1].0][pos[1].1 + 1] = '*';
            }
        } else {
            table[pos[0].0][pos[1].1] = '*';
            table[pos[1].0][pos[0].1] = '*';
        }

        let res: String = (1..n).fold(table[0].iter().collect::<String>(), |res, i| {
            res + "\n" + &table[i].iter().collect::<String>()
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
