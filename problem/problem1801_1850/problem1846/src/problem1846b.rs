use std::io::{stdout, Write, BufWriter};

fn main() {
    let t: usize = read();

    let symbols: Vec<char> = vec!['X', 'O', '+'];

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let mut field: Vec<Vec<char>> = vec![Vec::new(); 3];
        for i in 0..3 {
            field[i] = read_string().chars().collect::<Vec<char>>();
        }

        let mut index: usize = 3;
        for (i, s) in symbols.iter().enumerate() {
            for j in 0..3 {
                if field[j].iter().all(|&x| x == *s) {
                    index = i;
                }

                if [field[0][j], field[1][j], field[2][j]].iter().all(|&x| x == *s) {
                    index = i;
                }
            }

            if [field[0][0], field[1][1], field[2][2]].iter().all(|&x| x == *s) {
                index = i;
            }

            if [field[0][2], field[1][1], field[2][0]].iter().all(|&x| x == *s) {
                index = i;
            }
        }

        let res: &str = match index {
            0 => "X",
            1 => "O",
            2 => "+",
            _ => "DRAW",
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