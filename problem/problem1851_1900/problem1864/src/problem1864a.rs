use std::io::{stdout, Write, BufWriter};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let (x, y, n): (i32, i32, i32) = {
            let input: Vec<i32> = read_vec();
            (input[0], input[1], input[2])
        };

        let z: i32 = y - x;
        let h: i32 = n * (n - 1) / 2;

        let res: String =
            if z >= h {
                let mut r: Vec<i32> = vec![y];
                for i in 1..n-1 {
                    let j: usize = i as usize;
                    r.push(r[j-1] - i);
                }
                r.push(x);
                r.reverse();
                r.iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            }
            else {
                (-1).to_string()
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