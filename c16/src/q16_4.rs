use std::mem;

pub fn create_edges(
    n: usize,
    m: usize,
    reds: Vec<usize>,
    blues: Vec<usize>,
) -> Vec<(usize, usize, usize, usize, usize)> {
    let s: usize = 0;
    let t: usize = n + m + 1;
    let mut edges: Vec<(usize, usize, usize, usize, usize)> = Vec::new();

    for i in 0..n {
        edges.push((s, i + 1, 1, 0, reds[i]));
    }
    for i in 0..n {
        let r: usize = reds[i];
        for j in 0..m {
            let b: usize = blues[j];
            if is_prime(r, b) {
                edges.push((i + 1, j + n + 1, 1, r, b));
            }
        }
    }
    for i in 0..m {
        let b: usize = blues[i];
        edges.push((i + n + 1, t, 1, b, 0));
    }
    return edges
}

fn is_prime(_n: usize, _m: usize) -> bool {
    let mut n = _n;
    let mut m = _m;
    if n < m { mem::swap(&mut n, &mut m); }

    loop {
        let r = n % m;
        if r == 0 {
            if m == 1 {
                return false
            } else {
                return true
            }
        }
        n = m;
        m = r;
    }
}
