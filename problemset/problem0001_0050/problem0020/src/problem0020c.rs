use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{stdout, BufWriter, Write};

fn main() {
    let input: Vec<usize> = read_vec();
    let n: usize = input[0];
    let m: usize = input[1];

    let mut graph: Vec<Vec<(i64, usize)>> = vec![Vec::new(); n + 1];
    for _ in 0..m {
        let edge: Vec<usize> = read_vec();
        let a: usize = edge[0];
        let b: usize = edge[1];
        let w: i64 = edge[2] as i64;

        graph[a].push((w, b));
        graph[b].push((w, a));
    }

    let max: i64 = std::i64::MAX / 2;

    let mut dists = vec![max; n + 1];
    dists[1] = 0;

    let mut prevs = vec![0; n + 1];

    let mut heap: BinaryHeap<(Reverse<i64>, usize)> = BinaryHeap::new();
    heap.push((Reverse(0), 1));
    while let Some(x) = heap.pop() {
        let l: i64 = (x.0).0;
        let p: usize = x.1;
        for (x, y) in graph[p].iter() {
            let z: i64 = l + x;
            if z < dists[*y] {
                prevs[*y] = p;
                dists[*y] = z;
                heap.push((Reverse(z), *y));
            }
        }
    }

    let mut out = BufWriter::new(stdout().lock());

    if dists[n] == max {
        writeln!(out, "-1").unwrap();
        return;
    }

    let mut path: Vec<usize> = Vec::new();
    let mut now: usize = n;
    while now > 1 {
        path.push(now);
        now = prevs[now];
    }

    let res: String = path
        .iter()
        .rev()
        .fold(1.to_string(), |res, p| res + " " + &p.to_string());

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
