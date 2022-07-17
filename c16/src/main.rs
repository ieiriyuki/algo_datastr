use std::cmp::min;

fn main() {
    q16_1();
}

const INF: u32 = 1 << 30;


#[derive(Debug, Copy, Clone)]
struct Edge {
    from: usize,
    to: usize,
    cap: u32,
    index: usize,
    rev_index: usize,
}

struct Graph {
    list: Vec<Vec<Edge>>
}

impl Graph {
    // https://qiita.com/deepgreenAN/items/aa9d8b9d19384fa0a70a

    fn new(n: usize) -> Self {
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

    fn add_all_edge(&mut self, winput: Vec<(usize, usize, u32)>) {
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

fn fordfulkerson(
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

fn add_tail(
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

fn weighten(input: &Vec<(usize, usize)>) -> Vec<(usize, usize, u32)> {
    let mut winput: Vec<(usize, usize, u32)> = Vec::new();
    for (from, to) in input.iter() {
        winput.push((*from, *to, 1u32));
    }
    return winput
}

#[allow(unused_variables)]
fn q16_1() {
    println!("question 16-1");
    let n = 4usize;
    let women_id: Vec<usize> = vec![2, 3];
    let input: Vec<(usize, usize)> = vec![
        (0, 1),
        (1, 2),
        (1, 3),
    ];
    let input = add_tail(n, &input, &women_id);
    let winput: Vec<(usize, usize, u32)> = weighten(&input);
    let mut g: Graph = Graph::new(n + 1);
    g.add_all_edge(winput);
    println!("{}", fordfulkerson(&mut g, 0, n));

    let n = 4usize;
    let women_id: Vec<usize> = vec![3];
    let input: Vec<(usize, usize)> = vec![
        (0, 1),
        (0, 2),
        (1, 3),
        (2, 3),
    ];
    let input = add_tail(n, &input, &women_id);
    let winput: Vec<(usize, usize, u32)> = weighten(&input);
    let mut g: Graph = Graph::new(n + 1);
    g.add_all_edge(winput);
    println!("{}", fordfulkerson(&mut g, 0, n));

    let n = 10usize;
    let women_id: Vec<usize> = vec![7, 8, 9];
    let input: Vec<(usize, usize)> = vec![
        (0, 1),
        (0, 2),
        (0, 3),
        (0, 4),
        (1, 5),
        (2, 5),
        (5, 6),
        (6, 7),
        (6, 8),
        (3, 9),
        (4, 9),
    ];
    let input = add_tail(n, &input, &women_id);
    let winput: Vec<(usize, usize, u32)> = weighten(&input);
    let mut g: Graph = Graph::new(n + 1);
    g.add_all_edge(winput);
    println!("{}", fordfulkerson(&mut g, 0, n));

    let n = 6usize;
    let women_id: Vec<usize> = vec![4, 5];
    let input: Vec<(usize, usize)> = vec![
        (0, 1),
        (0, 2),
        (1, 3),
        (2, 3),
        (3, 4),
        (3, 5),
    ];
    let input = add_tail(n, &input, &women_id);
    let winput: Vec<(usize, usize, u32)> = weighten(&input);
    let mut g: Graph = Graph::new(n + 1);
    g.add_all_edge(winput);
    println!("{}", fordfulkerson(&mut g, 0, n));

    let n = 4usize;
    let women_id: Vec<usize> = vec![1, 2, 3];
    let input: Vec<(usize, usize)> = vec![
        (1, 2),
        (1, 3),
        (2, 3),
    ];
    let input = add_tail(n, &input, &women_id);
    let winput: Vec<(usize, usize, u32)> = weighten(&input);
    let mut g: Graph = Graph::new(n + 1);
    g.add_all_edge(winput);
    println!("{}", fordfulkerson(&mut g, 0, n));
}
