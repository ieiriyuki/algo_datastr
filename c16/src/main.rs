mod q16;
mod q16_3;

fn main() {
    q16_1();
    //q16_2();
    // Froid-Warshall and Ford-Fulkerson
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
    let input = q16::add_tail(n, &input, &women_id);
    let winput: Vec<(usize, usize, u32)> = q16::weighten(&input);
    let mut g: q16::Graph = q16::Graph::new(n + 1);
    g.add_all_edge(winput);
    println!("{}", q16::fordfulkerson(&mut g, 0, n));

    let n = 4usize;
    let women_id: Vec<usize> = vec![3];
    let input: Vec<(usize, usize)> = vec![
        (0, 1),
        (0, 2),
        (1, 3),
        (2, 3),
    ];
    let input = q16::add_tail(n, &input, &women_id);
    let winput: Vec<(usize, usize, u32)> = q16::weighten(&input);
    let mut g: q16::Graph = q16::Graph::new(n + 1);
    g.add_all_edge(winput);
    println!("{}", q16::fordfulkerson(&mut g, 0, n));

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
    let input = q16::add_tail(n, &input, &women_id);
    let winput: Vec<(usize, usize, u32)> = q16::weighten(&input);
    let mut g: q16::Graph = q16::Graph::new(n + 1);
    g.add_all_edge(winput);
    println!("{}", q16::fordfulkerson(&mut g, 0, n));

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
    let input = q16::add_tail(n, &input, &women_id);
    let winput: Vec<(usize, usize, u32)> = q16::weighten(&input);
    let mut g: q16::Graph = q16::Graph::new(n + 1);
    g.add_all_edge(winput);
    println!("{}", q16::fordfulkerson(&mut g, 0, n));

    let n = 4usize;
    let women_id: Vec<usize> = vec![1, 2, 3];
    let input: Vec<(usize, usize)> = vec![
        (1, 2),
        (1, 3),
        (2, 3),
    ];
    let input = q16::add_tail(n, &input, &women_id);
    let winput: Vec<(usize, usize, u32)> = q16::weighten(&input);
    let mut g: q16::Graph = q16::Graph::new(n + 1);
    g.add_all_edge(winput);
    println!("{}", q16::fordfulkerson(&mut g, 0, n));
}

fn q16_2() {
    //
}

