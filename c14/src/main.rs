use std::collections::{HashMap, VecDeque};

fn main() {
    q14_1();
    q14_2();
    q14_3();
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
    let dag = make_dag(HashMap::new(), 4, &e);
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
    let dag: DAG = make_dag(HashMap::new(), 5, &e);
    longest_path(dag);
}

fn make_dag(
    mut dag: DAG,
    n: usize,
    inputs: &Vec<(usize, usize)>
) -> DAG {
    // directed acyclic graph
    dag = (1..=n).map(|x| (x, Vec::<usize>::new())).collect();
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
    match bellman_ford(&wg, n) {
        Ok(x) => println!("{}", x),
        Err(x) => println!("{}", x),
    };

    let n = 2usize;
    let a = vec![
        (1, 2, 1),
        (2, 1, 1),
    ];
    let mut wg: WGraph = (1..=n).map(|x| (x, Vec::<WEdge>::new())).collect();
    wg = make_wgraph(&a, wg);
    match bellman_ford(&wg, n) {
        Ok(x) => println!("{}", x),
        Err(x) => println!("{}", x),
    };

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
    match bellman_ford(&wg, n) {
        Ok(x) => println!("{}", x),
        Err(x) => println!("{}", x),
    };

    let n = 6usize;
    let a = vec![
        (1, 2, -1000),
        (2, 3, -999),
        (3, 4, 1),
        (4, 2, -1),
        (4, 5, 2),
    ];
    let mut wg: WGraph = (1..=n).map(|x| (x, Vec::<WEdge>::new())).collect();
    wg = make_wgraph(&a, wg);
    match bellman_ford(&wg, n) {
        Ok(x) => println!("{}", x),
        Err(x) => println!("{}", x),
    };

    let n = 6usize;
    let a = vec![
        (1, 2, -1),
        (2, 3, -1),
        (3, 4, -1),
        (4, 2, -1),
        (2, 5, -1),
        (1, 5, -1000),
    ];
    let mut wg: WGraph = (1..=n).map(|x| (x, Vec::<WEdge>::new())).collect();
    wg = make_wgraph(&a, wg);
    match bellman_ford(&wg, n) {
        Ok(x) => println!("{}", x),
        Err(x) => println!("{}", x),
    };
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

fn bellman_ford(
    wg: &WGraph,
    n: usize,
) -> Result<i32, &'static str> {
    let ninf: i32 = -1 << 31;
    let mut order: Vec<usize> = wg.keys().map(|x| *x).collect();
    order.sort();

    let mut dist: HashMap<usize, i32> = (1..=n).map(|x| (x, ninf)).collect();
    let start = order[0];
    let val = dist.entry(start).or_insert(0);
    *val = 0;

    for i in 0..=n*n {
        let mut is_updated = false;

        for v in order.iter() {
            if dist[v] <= ninf / 2 {
                continue
            }

            for we in wg[v].iter() {
                let w0 = dist[v];
                if dist[&we.to] < w0 + we.weight {
                    if let Some(x) = dist.get_mut(&we.to) {
                        *x = w0 + we.weight;
                    }
                    is_updated = true;
                }
            }
        }
        if i == n*n && is_updated {
            return Err("inf")
        }
    }
    Ok(dist[&n])
}

fn q14_3() {
    println!("question 14-3");
    let n = 4usize;
    let e: Vec<(usize, usize)> = vec![
        (1, 2),
        (2, 3),
        (3, 4),
        (4, 1),
        //(1, 3),
    ];
    let dag = make_dag(HashMap::new(), n, &e);
    match hopscotch(&dag, 1, 3) {
        Ok(x) => println!("{}", x),
        Err(x) => println!("{}", x),
    }

    let n = 3usize;
    let e: Vec<(usize, usize)> = vec![
        (1, 2),
        (2, 3),
        (3, 1),
        //(1, 2),
    ];
    let dag = make_dag(HashMap::new(), n, &e);
    match hopscotch(&dag, 1, 2) {
        Ok(x) => println!("{}", x),
        Err(x) => println!("{}", x),
    }

    let n = 6usize;
    let e: Vec<(usize, usize)> = vec![
        (1, 2),
        (2, 3),
        (3, 4),
        (4, 5),
        (5, 1),
        (1, 4),
        (1, 5),
        (4, 6),
        //(1, 6),
    ];
    let dag = make_dag(HashMap::new(), n, &e);
    match hopscotch(&dag, 1, 6) {
        Ok(x) => println!("{}", x),
        Err(x) => println!("{}", x),
    }
}

fn hopscotch(
    dag: &DAG,
    s: usize,
    t: usize
) -> Result<i32, i32> {
    let mut dist: HashMap<usize, Vec<i32>>;
    dist = (1..=dag.len()).map(|x| (x, vec![-1; 3])).collect();
    if let Some(x) = dist.get_mut(&s) {
        x[0] = 0;
    }
    let mut que = VecDeque::<(usize, usize)>::new();
    que.push_back((s, 0));

    while ! que.is_empty() {
        let cur = que.pop_front().unwrap();
        let v = cur.0;
        let parity = cur.1;
        for nv in dag[&v].iter() {
            let np: usize = (parity + 1) % 3;
            if dist[nv][np] == -1 {
                let dist_i = dist[&v][parity];
                if let Some(x) = dist.get_mut(nv) {
                    println!("{}, {}, {:?}", nv, np, x);
                    x[np] = dist_i + 1;
                }
                que.push_back((*nv, np));
            }
        }
    }
    if dist[&t][0] == -1 {
        return Err(-1)
    } else {
        return Ok(dist[&t][0] / 3)
    }
}

#[allow(dead_code)]
fn dikstra(
    dag: &DAG,
    s: usize,
) -> i32 {
    let mut order: Vec<usize> = dag.keys().map(|x| *x).collect();
    order.sort();
    let mut is_used: HashMap<usize, bool> = order.iter().map(|x| (*x, false)).collect();
    let inf = 1 << 30;
    let mut dist: HashMap<usize, i32> = order.iter().map(|x| (*x, inf)).collect();
    if let Some(x) = dist.get_mut(&s) {
        *x = 0;
    }

    for _ in order.iter() {
        let mut min_dist = inf;
        let mut min_v = 0usize;

        for v in order.iter() {
            if (! is_used[v]) && dist[v] < min_dist {
                min_dist = dist[v];
                min_v = *v;
            }
        }

        if min_v == 0 { break; }
        for nv in dag[&min_v].iter() {
            let b = dist[&min_v] + 1;
            if dist[nv] > b {
                if let Some(x) = dist.get_mut(nv) {
                    *x = b;
                }
            }
        }
        if let Some(x) = is_used.get_mut(&min_v) {
            *x = true;
        }
    }
    let mut muximum = 0i32;
    for (k, v) in dist.iter() {
        if *k == s { continue; }
        if muximum < *v { muximum = *v; }
    }
    return muximum
}
