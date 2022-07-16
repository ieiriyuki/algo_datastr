use std::collections::HashMap;

fn main() {
    // question 15-1 is skipped
    q15_2();
    q15_3();
}

#[derive(Debug)]
struct UnionFind {
    par: HashMap<usize, usize>,
    siz: HashMap<usize, usize>,
}

#[allow(dead_code)]
impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind{
            par: (1..=n).zip(1..=n).collect(),
            siz: (1..=n).zip(vec![1; n]).collect(),
        }
    }

    fn root(&mut self, x: usize) -> usize {
        if self.par[&x] == x {
            x
        } else {
            let par = self.par[&x];
            let root = self.root(par);
            self.par.insert(x, root);
            self.par[&x]
        }
    }

    fn issame(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    fn unite(&mut self, mut x: usize, mut y: usize) -> bool {
        x = self.root(x);
        y = self.root(y);

        if x == y {
           return false
        }

        if self.siz[&x] < self.siz[&y] {
            std::mem::swap(&mut x, &mut y);
        }

        self.par.insert(y, x);
        let s = self.siz[&y];
        if let Some(x) = self.siz.get_mut(&x) {
            *x += s;
        }
        true
    }

    fn size(&mut self, x: usize) -> usize {
        let root = self.root(x);
        self.siz[&root]
    }
}

fn q15_2() {
    println!("question 15-1");
    let n: usize = 2;
    let edges: Vec<(usize, usize, i32)> = vec![
        (1, 2, 5),
    ];
    median_of_edge(n, edges);

    let n: usize = 4;
    let edges: Vec<(usize, usize, i32)> = vec![
        (1, 2, 1),
        (1, 3, 2),
        (1, 4, 3),
        (2, 3, 4),
        (3, 4, 6),
        (2, 4, 5),
    ];
    median_of_edge(n, edges);

    let n: usize = 8;
    let edges: Vec<(usize, usize, i32)> = vec![
        (1, 4, 767),
        (3, 1, 609),
        (8, 3, 426),
        (6, 5, 972),
        (8, 1, 607),
        (6, 4, 51),
        (5, 1, 683),
        (3, 6, 451),
        (3, 4, 630),
        (8, 7, 912),
        (3, 7, 43),
        (4, 7, 421),
        (3, 5, 582),
        (8, 4, 538),
        (5, 7, 832),
        (1, 6, 345),
        (8, 2, 608),
    ];
    median_of_edge(n, edges);

    let n: usize = 0;
    median_of_edge(n, Vec::<(usize, usize, i32)>::new());
}

fn median_of_edge(
    n: usize,
    edges: Vec<(usize, usize, i32)>,
) {
    if n == 0 { return }
    let mut ed = edges.clone();
    ed.sort_by(|a, b| a.2.cmp(&b.2));

    let mut uf = UnionFind::new(n);
    let mut vec_weight = Vec::<i32>::new();

    for (from, to, weight) in ed.iter() {
        if uf.issame(*from, *to) {
            continue
        }
        uf.unite(*from, *to);
        vec_weight.push(*weight);
    }
    vec_weight.sort();
    let median_weight = vec_weight[vec_weight.len() / 2];

    println!("{}", median_weight);
}

fn q15_3() {
    println!("question 15-3");

    let n: usize = 4;
    let mut edges: Vec<(usize, usize, i32)> = vec![
        (1, 2, 3),
        (1, 3, 3),
        (2, 3, 3),
        (2, 4, 3),
    ];
    no_alternative(n, edges);

    let n: usize = 4;
    let mut edges: Vec<(usize, usize, i32)> = vec![
        (1, 2, 3),
        (1, 3, 5),
        (2, 3, 3),
        (2, 4, 3),
    ];
    no_alternative(n, edges);

    let n: usize = 4;
    let mut edges: Vec<(usize, usize, i32)> = vec![
        (1, 2, 3),
        (1, 3, 1),
        (2, 3, 3),
        (2, 4, 3),
    ];
    no_alternative(n, edges);

    let n: usize = 3;
    let mut edges: Vec<(usize, usize, i32)> = vec![
        (1, 2, 1),
        (2, 3, 1),
        (1, 3, 1),
    ];
    no_alternative(n, edges);
}

fn no_alternative(
    n: usize,
    mut edges: Vec<(usize, usize, i32)>,
) {
    if n <= 1 { return }
    edges.sort_by(|a, b| a.2.cmp(&b.2));

    let mut uf = UnionFind::new(n);
    let mut edges_in_mst = Vec::<(usize, usize, i32)>::new();
    let mut ref_weight = 0i32;

    for (from, to, weight) in edges.iter() {
        if uf.issame(*from, *to) { continue }
        uf.unite(*from, *to);
        ref_weight += weight;
        edges_in_mst.push((*from, *to, *weight));
    }

    let mut necessaries = Vec::<(usize, usize, i32)>::new();

    for (from_n, to_n, weight_n) in edges_in_mst.iter() {
        let mut uf_i = UnionFind::new(n);
        let mut weight_i = 0i32;

        for (from, to, weight) in edges.iter() {
            if *from == *from_n && *to == *to_n { continue }
            if uf_i.issame(*from, *to) { continue }
            uf_i.unite(*from, *to);
            weight_i += weight;
        }

        if ref_weight != weight_i {
            necessaries.push((*from_n, *to_n, *weight_n));
        }
    }
    let mut m = 0usize;
    let mut cost = 0i32;
    for (_, _, w) in necessaries.iter() {
        m += 1;
        cost += w;
    }
    println!("{} {}", m, cost);
}
