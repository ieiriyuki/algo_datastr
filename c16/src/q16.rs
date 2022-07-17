use std::cmp::min;

const INF: u32 = 1 << 30;

#[derive(Debug, Copy, Clone)]
struct Edge {
    from: usize,
    to: usize,
    cap: u32,
    index: usize,
    rev_index: usize,
}

pub struct Graph {
    list: Vec<Vec<Edge>>
}


impl Graph {
    // https://qiita.com/deepgreenAN/items/aa9d8b9d19384fa0a70a

    pub fn new(n: usize) -> Self {
        Self { list: vec![Vec::new(); n] }
    }

    fn size(&self) -> usize {
        self.list.len()
    }

    fn edge<'a>(&'a mut self, e: Edge) -> &'a mut Edge{
        &mut self.list[e.from][e.index]
    }

    fn redge<'a>(&'a mut self, e: Edge) -> &'a mut Edge{
        &mut self.list[e.to][e.rev_index]
    }

    fn run_flow(&mut self, e: Edge, f: u32) {
        self.edge(e).cap -= f;
        self.redge(e).cap += f;
    }

    fn addedge(&mut self, from: usize, to: usize, cap: u32) {
        let last_from_index = self.list[from].len();
        let last_to_index = self.list[to].len();

        self.list[from].push(
            Edge{from, to, cap, index: last_from_index, rev_index: last_to_index}
        );
        self.list[to].push(
            Edge{from: to, to: from, cap: 0, index: last_to_index, rev_index: last_from_index}
        );
    }

    pub fn add_all_edge(&mut self, winput: Vec<(usize, usize, u32)>) {
        for (from, to, cap) in winput.iter() {
            self.addedge(*from, *to, *cap);
        }
    }
}

fn fodfs(
    g: &mut Graph,
    seen: &mut Vec<bool>,
    v: usize,
    t: usize,
    f: u32
) -> Option<u32> {
    if v == t { return Some(f) }

    seen[v] = true;
    let from_v_edges = g.list[v].clone();

    for e in from_v_edges.into_iter() {
        if seen[e.to] { continue }

        if g.edge(e).cap == 0 { continue }

        let e_cap = g.edge(e).cap;
        match fodfs(g, seen, e.to, t, min(f, e_cap)) {
            None =>  { continue },
            Some(flow) => {
                g.run_flow(e, flow);
                return Some(flow);
            },
        }
    }

    None
}

pub fn fordfulkerson(
    g: &mut Graph,
    s: usize,
    t: usize
) -> u32 {
    let mut max_flow = 0_u32;
    loop {
        let mut seen = vec![false; g.size()];
        match fodfs(g, &mut seen, s, t, INF) {
            None => { break },
            Some(flow) => {
                max_flow += flow;
            },
        }
    }
    max_flow
}

pub fn add_tail(
    n: usize,
    input: &Vec<(usize, usize)>,
    women_id: &Vec<usize>,
) -> Vec<(usize, usize)> {
    let mut output: Vec<(usize, usize)> = input.clone();
    for woman in women_id.iter() {
        output.push((*woman, n));
    }

    output
}

pub fn weighten(input: &Vec<(usize, usize)>) -> Vec<(usize, usize, u32)> {
    let mut winput: Vec<(usize, usize, u32)> = Vec::new();
    for (from, to) in input.iter() {
        winput.push((*from, *to, 1u32));
    }
    return winput
}
