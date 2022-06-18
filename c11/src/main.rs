use std::collections::{HashSet, HashMap};

fn main() {
    println!("question 11-1");
    let a: Vec<(usize, usize)> = vec![
        (1, 3),
        (2, 7),
        (3, 4),
        (4, 5),
        (4, 6),
        (5, 6),
        (6, 7),
    ];
    q11_1(&a);
}

fn q11_1(v_: &Vec<(usize, usize)>) -> i32 {
    let m = v_.len();
    let n = get_num_of_elem(v_);
    println!("{}, {}", m, n);
    let mut uf = UnionFind::new(n);
    println!("{:?}", uf);

    for (a, b) in v_.into_iter() {
        let _ = uf.unite(*a, *b);
        println!("{:?}", uf);
    }
    return 1
}

fn get_num_of_elem(v: &Vec<(usize, usize)>) -> usize {
    let mut x: HashSet<usize> = HashSet::new();
    for item in v.into_iter() {
        let (a, b) = item;
        x.insert(*a);
        x.insert(*b);
    }
    return x.len()
}

#[derive(Debug)]
struct UnionFind {
    par: HashMap<usize, usize>,
    siz: HashMap<usize, usize>,
}

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
