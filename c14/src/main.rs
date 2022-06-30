use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    q14_1();
}

type DAG = HashMap<usize, Vec<usize>>;

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
    dag = reverse_dag(dag);
    let mut degree: HashMap<usize, i32> = dag.keys().map(|x| (*x, 0)).collect();
    degree = count_degree(degree, &e);
    println!("{:?}", degree);
    let mut order = Vec::<usize>::new();
    order = bfs_topsort(&dag, degree, order);
    println!("{:?}", order);

    let e: Vec<(usize, usize)> = vec![
        (2, 3),
        (4, 5),
        (5, 6),
    ];
    let mut dag: DAG = HashMap::new();
    dag = make_dag(dag, &e);
    println!("{:?}", dag);

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
    println!("{:?}", dag);

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

fn reverse_dag(mut dag: DAG) -> DAG {
    let mut temp: DAG = HashMap::new();
    for key in dag.keys() {
        temp.insert(*key, Vec::new());
    }
    for (key, val) in dag.iter() {
        for elem in val.iter() {
            let comes_from = temp.entry(*elem).or_insert(Vec::new());
            comes_from.push(*key);
        }
    }
    dag = temp.clone();
    return dag
}

fn count_degree(
    mut degree: HashMap<usize, i32>,
    inputs: &Vec<(usize, usize)>,
) -> HashMap<usize, i32> {
    for (from, _) in inputs.iter() {
        let c = degree.get_mut(&from).unwrap();
        *c += 1;
    }
    return degree
}

fn bfs_topsort(
    dag: &DAG,
    mut degree: HashMap<usize, i32>,
    mut order: Vec<usize>,
) -> Vec<usize> {
    let mut queue = VecDeque::<usize>::new();
    for (key, val) in degree.iter() {
        if *val == 0 {
            queue.push_back(*key);
        }
    }

    while ! queue.is_empty() {
        let cur_v = queue.pop_front().unwrap();
        order.push(cur_v);

        for next_v in dag[&cur_v].iter() {
            let c = degree.get_mut(next_v).unwrap();
            *c -= 1;
            if degree[next_v] == 0 {
                queue.push_back(*next_v)
            }
        }
    }
    order.reverse();
    return order
}

#[derive(Debug)]
struct Edge {
    to: usize,
    weight: i32,
}
