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
    println!("n bridges: {}", q11_1(&a));
    let a = vec![
        (1, 2),
        (1, 3),
        (2, 3),
    ];
    println!("n bridges: {}", q11_1(&a));
    let a = vec![
        (1, 2),
        (2, 3),
        (3, 4),
        (4, 5),
        (5, 6),
    ];
    println!("n bridges: {}", q11_1(&a));

    println!("\nquestion 11-2");
}

fn q11_1(v_: &Vec<(usize, usize)>) -> i32 {
    let m = v_.len();
    let n = get_num_of_elem(v_);
    let mut n_bridge= 0;
    let mut bridges: Vec<(usize, usize)> = Vec::new();
    // v_ 中の辺 i を除いたときに, 全ての点の根が同じかどうか
    for i in 0..m {
        let mut v = v_.clone();
        v.remove(i);
        // i 以外の辺で UnionFind を作る
        let mut uf = UnionFind::new(n);
        for (a, b) in v.into_iter() {
            uf.unite(a, b);
        }
        // 全ての点の根が同じだったら, 橋ではない
        let mut are_all_same = true;
        for j in 1..n {
            for k in j..=n {
                are_all_same = uf.issame(j, k);
                if ! are_all_same {
                    break
                }
            }
            if ! are_all_same {
                break
            }
        }
        if ! are_all_same {
            bridges.push(v_[i]);
            n_bridge += 1;
        }
    }
    println!("{:?}", bridges);
    return n_bridge
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
