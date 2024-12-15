use std::io::{stdout, Write, BufWriter};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let (n, m): (usize, usize) = {
            let input: Vec<usize> = read_vec();
            (input[0], input[1])
        };

        let mut a: Vec<Vec<u64>> = vec![vec![0; m]; n];
        for i in 0..n {
            let input: Vec<u64> = read_vec();
            a[i] = input;
        }

        let (u, v): (i32, i32) = (n as i32, m as i32);
        let mut res: String = String::new();
        for x in 0..n {
            let mut row: String = String::new();

            for y in 0..m {
                let mut max: u64 = 0;
                for (dx, dy) in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
                    let (nx, ny): (i32, i32) = (x as i32 + dx, y as i32 + dy);
                    if 0 <= nx && nx < u && 0 <= ny && ny < v {
                        max = max.max(a[nx as usize][ny as usize]);
                    }
                }

                a[x][y] = a[x][y].min(max);
                if y == 0 {
                    row = a[x][y].to_string();
                }
                else {
                    row += &(" ".to_owned() + &a[x][y].to_string());
                }
            }

            if x == 0 {
                res = row;
            }
            else {
                res += &("\n".to_owned() + &row);
            }
        }

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