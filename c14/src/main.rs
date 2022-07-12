use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    q14_1();
    q14_2();
}

type DAG = HashMap<usize, Vec<usize>>;
#[derive(Debug)]
struct WEdge {
    to: usize,
    weight: i32,
}
type WGraph = HashMap<usize, Vec<WEdge>>;


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
    longest_path(dag);

    //すべてのノードが登場しないと、インデックスでエラる

    let e: Vec<(usize, usize)> = vec![
        (5, 3),
        (2, 3),
        (2, 4),
        (5, 2),
        (5, 1),
        (1, 4),
        (4, 3),
        (1, 3),
    ];
    let mut dag: DAG = HashMap::new();
    dag = make_dag(dag, &e);
    longest_path(dag);
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
    mut dp: Vec<i32>,
) -> (Vec<i32>, i32) {
    if dp[v - 1] != -1 {
        let res = dp[v - 1];
        return (dp, res)
    }

    for next_v in dag[&v].iter() {
        let res: i32;
        (dp, res) = dfs(dag, *next_v, dp);
        if dp[v - 1] < res + 1 {
            dp[v - 1] = res + 1;
        }
    }
    let mut res = dp[v - 1];
    if res < 0 {
        res = 0;
    }
    dp[v - 1] = res;
    (dp, res)
}

fn longest_path(
    dag: DAG,
) -> i32 {
    let mut dp: Vec<i32> = vec![-1; dag.len()];
    let mut x: i32 = 0;
    for v in dag.keys() {
        let temp: i32;
        (dp, temp) = dfs(&dag, *v, dp);
        if x < temp {
            x = temp;
        }
    }
    println!("{}, {:?}", x, dp);
    x
}

fn q14_2() {
    println!("question 14-2");
    let n = 3usize;
    let a = vec![
        (1, 2, 4),
        (2, 3, 3),
        (1, 3, 5),
    ];
    let mut wg: WGraph = (1..=n).map(|x| (x, Vec::<WEdge>::new())).collect();
    wg = make_wgraph(&a, wg);
    println!("{:?}", wg);

    let n = 2usize;
    let a = vec![
        (1, 2, 1),
        (2, 1, 1),
    ];
    let mut wg: WGraph = (1..=n).map(|x| (x, Vec::<WEdge>::new())).collect();
    wg = make_wgraph(&a, wg);
    println!("{:?}", wg);

    let n = 6usize;
    let a = vec![
        (1, 2, -1000),
        (2, 3, -1000),
        (3, 4, -1000),
        (4, 5, -1000),
        (5, 6, -1000),
    ];
    let mut wg: WGraph = (1..=n).map(|x| (x, Vec::<WEdge>::new())).collect();
    wg = make_wgraph(&a, wg);
    println!("{:?}", wg);
}

fn make_wgraph(
    edge: &Vec<(usize, usize, i32)>,
    mut wg: WGraph,
) -> WGraph {
    // weighted directed graph
    for item in edge.iter() {
        let (node, path, weight) = *item;
        let we = WEdge {to: path, weight: weight};
        let vc = wg.entry(node).or_insert(Vec::<WEdge>::new());
        vc.push(we);
    }
    return wg
}

fn bellman_ford() -> Result<i32, &'static str> {
    let inf: i32 = 1 << 31;
    let mut dist: HashMap<usize, i32> = (0..99).map(|x| (x, inf)).collect();

    
    Err("inf")
}
