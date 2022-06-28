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

fn bfs(g: &Vec<Vec<usize>>, v: usize, seen: &mut Vec<bool>) {

}

fn c13_3() {
    println!("code 13-3");
}
