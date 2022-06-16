use std::collections::HashSet;

fn main() {
    println!("question 11-1");
    let a: Vec<(i32, i32)> = vec![
        (1, 3),
        (2, 7),
        (3, 4),
        (4, 5),
        (4, 6),
        (5, 6),
        (6, 7),
    ];
    q11_1(&a);
}

fn q11_1(v_: &Vec<(i32, i32)>) -> i32 {
    let m = v_.len();
    let n = get_num_of_elem(v_) as i32;
    println!("{}, {}", m, n);
    return 1
}

fn get_num_of_elem(v: &Vec<(i32, i32)>) -> usize {
    let mut x: HashSet<i32> = HashSet::new();
    for item in v.into_iter() {
        let (a, b) = item;
        x.insert(*a);
        x.insert(*b);
    }
    return x.len()
}
