use std::collections::{HashSet, VecDeque};

fn main() {
    let g: Vec<Vec<usize>> = vec![
        vec![5],
        vec![3, 6],
        vec![5, 7],
        vec![0, 7],
        vec![1, 2, 6],
        vec![],
        vec![7],
        vec![0],
    ];
    c13_2(&g);
    c13_3(&g);

    do_q13_1();
    do_q13_2();
    do_q13_3();
    // question 13-4, データの用意が面倒なのでパス
    do_q13_5();
    do_q13_6();
}

fn dfs(g: &Vec<Vec<usize>>, v: usize, seen: &mut Vec<bool>) {
    seen[v] = true;
    println!("{}", v);
    for next_v in g[v].iter() {
        if seen[*next_v] {
            continue
        }
        dfs(g, *next_v, seen);
    }
}

fn c13_2(g: &Vec<Vec<usize>>) {
    println!("code 13-2");
    let mut seen = vec![false; 8];
    for i in 0..7 {
        dfs(g, i as usize, &mut seen);
    }
}

fn bfs(g: &Vec<Vec<usize>>, s: usize) -> usize {
    let n = g.len();
    let mut dist: Vec<i32> = vec![-1; n];
    let mut queue = VecDeque::<usize>::new();

    dist[s] = 0;
    queue.push_back(s);
    while ! queue.is_empty() {
        let v = queue.pop_front().unwrap();
        queue.pop_back();

        for i in g[v].iter() {
            if dist[*i] != -1 {
                continue
            }
            println!("{}", v);
            dist[*i] = dist[v] + 1;
            queue.push_back(*i);
        }
    }

    return (*dist.iter().max().unwrap()).try_into().unwrap()
}

fn c13_3(g: &Vec<Vec<usize>>) {
    println!("code 13-3");
    for i in 0..7 {
        bfs(g, i);
    }
}

fn make_graph(mut g: Vec<Vec<usize>>, inputs: &Vec<(usize, usize)>) -> Vec<Vec<usize>> {
    // undirected graph
    let mut x: HashSet<usize> = HashSet::new();
    for item in inputs.iter() {
        let (node, path) = *item;
        x.insert(node);
        x.insert(path);
    }
    for _i in 0..x.len() {
        g.push(Vec::<usize>::new());
    }
    for item in inputs.iter() {
        let (node, path) = *item;
        g[node].push(path);
        g[path].push(node);
    }
    return g
}

fn q13_1(g: &Vec<Vec<usize>>) -> usize {
    println!("{:?}", g);
    let mut roots: Vec<i32> = vec![-1; g.len()];
    for i in 0..g.len() {
        roots = bfs_rootfind(g, roots, i);
    }
    roots.sort();
    roots.dedup();
    println!("{:?}", roots);
    return roots.len()
}

fn do_q13_1() {
    println!("question 13-1");
    let a: Vec<(usize, usize)> = vec![
        (0, 2),
        (1, 3),
        (2, 6),
        (4, 5),
    ];
    let mut g: Vec<Vec<usize>> = Vec::new();
    g = make_graph(g, &a);
    println!("{}", q13_1(&g));
    let a: Vec<(usize, usize)> = vec![
        (0, 1),
        (1, 2),
        (2, 3),
        (3, 4),
        (4, 5),
    ];
    let mut g: Vec<Vec<usize>> = Vec::new();
    g = make_graph(g, &a);
    println!("{}", q13_1(&g));
}

fn bfs_rootfind(g: &Vec<Vec<usize>>, mut roots: Vec<i32>, v: usize) -> Vec<i32> {
    let mut queue = VecDeque::<usize>::new();
    if roots[v] == -1 {
        roots[v] = v as i32;
    }
    queue.push_back(v);
    while ! queue.is_empty() {
        let cur = queue.pop_front().unwrap();

        for i in g[cur].iter() {
            if roots[*i] != -1 {
                continue
            }
            roots[*i] = roots[cur];
            queue.push_back(*i);
        }
    }
    return roots
}

fn do_q13_2() {
    println!("question 13-2");
    let a: Vec<(usize, usize)> = vec![
        (0, 1),
        (1, 2),
        (2, 3),
        (4, 5),
        (2, 5),
    ];
    let mut g: Vec<Vec<usize>> = Vec::new();
    g = make_dag(g, &a);
    println!("{}", q13_2(&g, 0, 3));
    println!("{}", q13_2(&g, 3, 5));
}

fn make_dag(mut g: Vec<Vec<usize>>, inputs: &Vec<(usize, usize)>) -> Vec<Vec<usize>> {
    // directed acyclic graph
    let mut x: HashSet<usize> = HashSet::new();
    for item in inputs.iter() {
        let (node, path) = *item;
        x.insert(node);
        x.insert(path);
    }
    for _i in 0..x.len() {
        g.push(Vec::<usize>::new());
    }
    for item in inputs.iter() {
        let (node, path) = *item;
        g[node].push(path);
    }
    return g
}

fn q13_2(g: &Vec<Vec<usize>>, s: usize, t: usize) -> bool {
    println!("{:?}", g);
    let mut is_reachable = vec![false; g.len()];
    is_reachable = bfs_reachable(g, is_reachable, s);
    return is_reachable[t]
}

fn bfs_reachable(
    g: &Vec<Vec<usize>>,
    mut is_reachable: Vec<bool>,
    s: usize,
) -> Vec<bool> {
    let mut queue = VecDeque::<usize>::new();
    if ! is_reachable[s] {
        is_reachable[s] = true;
    }
    queue.push_back(s);
    while ! queue.is_empty() {
        let cur = queue.pop_front().unwrap();
        for i in g[cur].iter() {
            if is_reachable[*i] {
                continue
            }
            is_reachable[*i] = true;
            queue.push_back(*i);
        }
    }
    return is_reachable
}

fn do_q13_3() {
    println!("qustion 13-3");
    let a: Vec<(usize, usize)> = vec![
        (0, 1),
        (0, 5),
        (1, 2),
        (2, 3),
        (2, 5),
        (3, 4),
        (4, 5),
    ];
    let mut g: Vec<Vec<usize>> = Vec::new();
    g = make_graph(g, &a);
    println!("{}", q13_3(&g)); // should be true
    let a: Vec<(usize, usize)> = vec![
        (0, 1),
        (0, 5),
        (1, 2),
        (2, 3),
        (2, 5),
        (3, 4),
        (4, 5),
        (4, 2),
    ];
    let mut g: Vec<Vec<usize>> = Vec::new();
    g = make_graph(g, &a);
    println!("{}", q13_3(&g)); // should be false
}

fn q13_3(g: &Vec<Vec<usize>>) -> bool {
    let mut labels = vec!['U'; g.len()];
    for i in 0..g.len() {
        labels = bfs_label(g, labels, i);
    }
    println!("{:?}", labels);
    for i in 0..g.len() {
        for j in g[i].iter() {
            if labels[i] == labels[*j] {
                return false
            }
        }
    }
    true
}

fn bfs_label(
    g: &Vec<Vec<usize>>,
    mut labels: Vec<char>,
    s: usize,
) -> Vec<char> {
    let mut queue = VecDeque::<usize>::new();
    if labels[s] == 'U' {
        labels[s] = 'A';
    }
    queue.push_back(s);
    while ! queue.is_empty() {
        let cur = queue.pop_front().unwrap();

        for i in g[cur].iter() {
            if labels[s] == labels[*i] {
                return labels
            }
            if labels[*i] == 'U' {
                if labels[s] == 'A' {
                    labels[*i] = 'B'
                } else if labels[s] == 'B' {
                    labels[*i] = 'A'
                }
            }
            queue.push_back(*i);
        }
    }
    return labels
}

fn do_q13_5() {
    println!("question 13-5");
    let a: Vec<(usize, usize)> = vec![
        (0, 5),
        (1, 3),
        (1, 6),
        (2, 5),
        (2, 7),
        (3, 0),
        (3, 7),
        (4, 1),
        (4, 2),
        (4, 6),
        (6, 7),
        (7, 0),
    ];
    let mut dag: Vec<Vec<usize>> = Vec::new();
    dag = make_dag(dag, &a);
    println!("{:?}", dag);
    q13_5(&dag);
}

fn q13_5(dag: &Vec<Vec<usize>>) {
    for i in 0..dag.len() {
        let mut order = Vec::<usize>::new();
        order = bfs_topsort(dag, order, i);
        println!("{:?}", order);
        let mut visit = vec![false; dag.len()];
        order.reverse();
        let mut final_order = Vec::<usize>::new();
        for j in order.into_iter() {
            if ! visit[j] {
                final_order.push(j);
                visit[j] = true;
            }
        }
        if final_order.len() == dag.len() {
            final_order.reverse();
            println!("{:?}", final_order);
            return
        }
    }
}

fn bfs_topsort(
    dag: &Vec<Vec<usize>>,
    mut order: Vec<usize>,
    s: usize
) -> Vec<usize> {
    let mut queue = VecDeque::<usize>::new();
    order.push(s);
    queue.push_back(s);
    while ! queue.is_empty() {
        let cur = queue.pop_front().unwrap();
        for i in dag[cur].iter() {
            /*if visit[*i] {
                continue
            }*/
            order.push(*i);
            queue.push_back(*i);
        }
    }
    return order
}

fn do_q13_6() {
    println!("question 13-6");
    let a: Vec<(usize, usize)> = vec![
        (0, 5),
        (1, 3),
        (1, 6),
        (2, 5),
        (2, 7),
        (3, 0),
        (3, 7),
        (4, 1),
        (4, 2),
        (4, 6),
        (6, 7),
        (7, 0),
    ];
    let mut dag: Vec<Vec<usize>> = Vec::new();
    dag = make_dag(dag, &a);
    q13_6(&dag); // no cycle
    let a: Vec<(usize, usize)> = vec![
        (0, 1),
        (1, 2),
        (1, 3),
        (2, 4),
        (3, 0),
    ];
    let mut dag: Vec<Vec<usize>> = Vec::new();
    dag = make_dag(dag, &a);
    q13_6(&dag); // cycle
}

fn q13_6(dag: &Vec<Vec<usize>>){
    let mut seen = vec![false; dag.len()];
    let mut path = Vec::<usize>::new();
    for i in 0..dag.len() {
        seen = dfs_cycle(dag, &mut seen, i, i, &mut path);
    }
}

fn dfs_cycle(
    dag: &Vec<Vec<usize>>,
    seen: &mut Vec<bool>,
    v: usize,
    par: usize,
    path: &mut Vec<usize>,
) -> Vec<bool> {
    seen[v] = true;
    path.push(v);
    for next_v in dag[v].iter() {
        if par == *next_v {
            path.push(*next_v);
            println!("{}, {}, {}, {:?}", par, v, *next_v, path);
        }
        if seen[*next_v] {
            continue
        }
        dfs_cycle(dag, seen, *next_v, par, path);
    }
    return seen.to_vec()
}
