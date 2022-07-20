use std::cmp::min;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Edge {
    to: usize,
    cap: isize,
    rev: usize,
}

#[derive(Clone, Debug)]
pub struct FordFulkerson {
    g: Vec<Vec<Edge>>,
    used: Vec<bool>,
}

// https://atcoder.jp/contests/arc085/submissions/25395107
impl FordFulkerson{
    pub fn new(n: usize) -> FordFulkerson {
        let g: Vec<Vec<_>> = vec![vec![]; n];
        let used: Vec<bool> = vec![false; n];
        FordFulkerson {g, used}
    }

    pub fn add_edge(&mut self, from: usize, to: usize, cap: isize) {
        let to_edge = Edge {
            to, cap, rev: self.g[to].len()
        };
        self.g[from].push(to_edge);
        let from_edge = Edge {
            to: from, cap: 0, rev: self.g[from].len() - 1
        };
        self.g[to].push(from_edge);
    }

    fn dfs(&mut self, v: usize, t: usize, f: isize) -> isize {
        if v == t { return f }

        self.used[v] = true;
        for i in 0..self.g[v].len() {
            let edge = self.g[v][i];
            if ! self.used[edge.to] && 0 < edge.cap {
                let d = self.dfs(edge.to, t, min(f, edge.cap));
                if 0 < d {
                    self.g[v][i].cap -= d;
                    self.g[edge.to][edge.rev].cap += d;
                    return d
                }
            }
        }
        0
    }

    pub fn max_flow(&mut self, s: usize, t: usize) -> isize {
        let mut flow = 0;
        loop {
            self.used = vec![false; self.used.len()];
            let f = self.dfs(s, t, std::isize::MAX);
            if f == 0 {
                return flow
            }
            flow += f;
        }
    }
}
