use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    q14_1();
}

type DAG = HashMap<usize, Vec<usize>>;
type Seen = HashMap<usize, bool>;

fn q14_1() {
    println!("question 14-1");
    let e: Vec<(usize, usize)> = vec![
        (1, 2),
        (1, 3),
        (3, 2),
        (2, 4),
        (3, 4),
    ];
    let mut dag: DAG = HashMap::new();
    dag = make_dag(dag, &e);
    let mut order = Vec::<usize>::new();
    order = topo_sort(&dag, order);
    println!("{:?}", order);

}

fn make_dag(
    mut dag: DAG,
    inputs: &Vec<(usize, usize)>
) -> DAG {
    // directed acyclic graph
    let mut x: HashSet<usize> = HashSet::new();
    for item in inputs.iter() {
        let (node, path) = *item;
        x.insert(node);
        x.insert(path);
    }
    for node in x.into_iter() {
        dag.insert(node, Vec::<usize>::new());
    }
    for item in inputs.iter() {
        let (node, path) = *item;
        dag.get_mut(&node).unwrap().push(path);
    }
    return dag
}

fn dfs(
    dag: &DAG,
    v: usize,
    seen: &mut Seen,
    mut order: Vec<usize>,
) -> Vec<usize> {
    if let Some(x) = seen.get_mut(&v) {
        *x = true;
    }
    for next_v in dag[&v].iter() {
        if seen[next_v] {
            continue
        }
        order = dfs(dag, *next_v, seen, order);
    }
    order.push(v);
    order
}

fn topo_sort(
    dag: &DAG,
    mut order: Vec<usize>,
) -> Vec<usize> {
    let mut seen: Seen = dag.keys().map(|x| (*x, false)).collect();

    for v in dag.keys() {
        if seen[v] {
            continue;
        }
        order = dfs(dag, *v, &mut seen, order);
    }
    order.reverse();
    order
}

fn longest_path(
    dag: DAG,
    _order: Vec<usize>,
) -> usize {
    // this is wrong
    let order: HashMap<usize, usize> =
        _order.iter().enumerate().map(|(idx, &val)| (val, idx)).collect()
    ;
    println!("{:?}", order);
    println!("{:?}", dag);
    let mut dp = vec![0; order.len() + 1];
    for (key, i) in order.iter() {
        for _j in dag.get(key).unwrap().iter() {
            let j = order[_j];
            dp[j] += dp[*i] + 1;
        }
    }
    dp[order.len()] = dp[order.len() - 1];
    println!("{:?}", dp);
    dp.sort();
    dp[order.len()]
}

#[derive(Debug)]
struct Edge {
    to: usize,
    weight: i32,
}
