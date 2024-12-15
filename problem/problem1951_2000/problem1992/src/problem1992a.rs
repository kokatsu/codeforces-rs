use std::io::{stdout, Write, BufWriter};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let (a, b, c): (u32, u32, u32) = {
            let input: Vec<u32> = read_vec();
            (input[0], input[1], input[2])
        };

        let (x, y, z): (u32, u32, u32) = (0..5)
            .fold((a, b, c), |(x, y, z), _| {
                let u: u32 = (x + 1) * y * z;
                let v: u32 = x * (y + 1) * z;
                let w: u32 = x * y * (z + 1);
                if u >= v && u >= w {
                    (x + 1, y, z)
                }
                else if v >= u && v >= w {
                    (x, y + 1, z)
                }
                else {
                    (x, y, z + 1)
                }
            });

        let res: u32 = x * y * z;

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