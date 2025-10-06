use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let (n, m): (usize, usize) = {
            let input: Vec<usize> = read_vec();
            (input[0], input[1])
        };

        let s: Vec<Vec<char>> = (0..n).map(|_| read_string().chars().collect()).collect();

        let r: char = 'R';
        let w: char = 'W';

        let rw: bool = (0..n).all(|i| {
            (0..m).all(|j| s[i][j] == if (i + j) % 2 == 0 { r } else { w } || s[i][j] == '.')
        });

        let wr: bool = (0..n).all(|i| {
            (0..m).all(|j| s[i][j] == if (i + j) % 2 == 0 { w } else { r } || s[i][j] == '.')
        });

        let res: String = if rw {
            let grid = (0..n)
                .map(|i| {
                    (0..m)
                        .map(|j| if (i + j) % 2 == 0 { r } else { w })
                        .collect::<String>()
                })
                .collect::<Vec<String>>()
                .join("\n");
            format!("YES\n{}", grid)
        } else if wr {
            let grid = (0..n)
                .map(|i| {
                    (0..m)
                        .map(|j| if (i + j) % 2 == 0 { w } else { r })
                        .collect::<String>()
                })
                .collect::<Vec<String>>()
                .join("\n");
            format!("YES\n{}", grid)
        } else {
            String::from("NO")
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
