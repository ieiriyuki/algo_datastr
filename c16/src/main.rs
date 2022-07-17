use crate::q16_4::create_edges;
use crate::q16_5::{word2vec, show_field, odds_evens};
use crate::q16_6::{FordFulkerson};

mod q16;
mod q16_4;
mod q16_5;
mod q16_6;

fn main() {
    q16_1();
    println!("q16_2 skip");
    // Froid-Warshall and Ford-Fulkerson
    println!("q16_3 skip");
    q16_4();
    q16_5();
    q16_6();
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

#[allow(dead_code)]
fn q16_2() {
    //
}

fn q16_4() {
    println!("question 16-4");
    let n: usize = 4;
    let m: usize = 3;
    let reds: Vec<usize> = vec![
        2, 6, 6, 15,
    ];
    let blues: Vec<usize> = vec![
        2, 3, 5,
    ];
    let input: Vec<(usize, usize, usize, usize, usize)> = create_edges(n, m, reds, blues);
    let mut g: q16::Graph = q16::Graph::new(n + m + 2);
    g.add_all_edge(
        input.iter()
        .map(|x| (x.0, x.1, x.2 as u32))
        .collect()
    );
    println!("{}", q16::fordfulkerson(&mut g, 0, n + m + 1));

    let n: usize = 2;
    let m: usize = 3;
    let reds: Vec<usize> = vec![
        4, 9,
    ];
    let blues: Vec<usize> = vec![
        8, 16, 32,
    ];
    let input: Vec<(usize, usize, usize, usize, usize)> = create_edges(n, m, reds, blues);
    let mut g: q16::Graph = q16::Graph::new(n + m + 2);
    g.add_all_edge(
        input.iter()
        .map(|x| (x.0, x.1, x.2 as u32))
        .collect()
    );
    println!("{}", q16::fordfulkerson(&mut g, 0, n + m + 1));

    let n: usize = 4;
    let m: usize = 2;
    let reds: Vec<usize> = vec![
        4, 9, 11, 13,
    ];
    let blues: Vec<usize> = vec![
        5, 7,
    ];
    let input: Vec<(usize, usize, usize, usize, usize)> = create_edges(n, m, reds, blues);
    let mut g: q16::Graph = q16::Graph::new(n + m + 2);
    g.add_all_edge(
        input.iter()
        .map(|x| (x.0, x.1, x.2 as u32))
        .collect()
    );
    println!("{}", q16::fordfulkerson(&mut g, 0, n + m + 1));

    let n: usize = 5;
    let m: usize = 5;
    let reds: Vec<usize> = vec![
        2, 3, 5, 1001, 1001,
    ];
    let blues: Vec<usize> = vec![
        7, 11, 13, 30, 30,
    ];
    let input: Vec<(usize, usize, usize, usize, usize)> = create_edges(n, m, reds, blues);
    let mut g: q16::Graph = q16::Graph::new(n + m + 2);
    g.add_all_edge(
        input.iter()
        .map(|x| (x.0, x.1, x.2 as u32))
        .collect()
    );
    println!("{}", q16::fordfulkerson(&mut g, 0, n + m + 1));

    let n: usize = 10;
    let m: usize = 10;
    let reds: Vec<usize> = vec![
        2, 3, 5, 7, 9, 11, 13, 15, 17, 29,
    ];
    let blues: Vec<usize> = vec![
        4, 6, 10, 14, 18, 22, 26, 30, 34, 38,
    ];
    let input: Vec<(usize, usize, usize, usize, usize)> = create_edges(n, m, reds, blues);
    let mut g: q16::Graph = q16::Graph::new(n + m + 2);
    g.add_all_edge(
        input.iter()
        .map(|x| (x.0, x.1, x.2 as u32))
        .collect()
    );
    println!("{}", q16::fordfulkerson(&mut g, 0, n + m + 1));

    let n: usize = 20;
    let m: usize = 20;
    let reds: Vec<usize> = vec![
        195, 144, 903, 63, 137, 513, 44, 626, 75, 473, 876, 421, 568, 519, 755, 840, 374, 368, 570, 872,
    ];
    let blues: Vec<usize> = vec![
        363, 650, 155, 265, 64, 26, 426, 391, 15, 421, 373, 984, 564, 54, 823, 477, 565, 866, 879, 638,
    ];
    let input: Vec<(usize, usize, usize, usize, usize)> = create_edges(n, m, reds, blues);
    let mut g: q16::Graph = q16::Graph::new(n + m + 2);
    g.add_all_edge(
        input.iter()
        .map(|x| (x.0, x.1, x.2 as u32))
        .collect()
    );
    println!("{}", q16::fordfulkerson(&mut g, 0, n + m + 1));

    let n: usize = 100;
    let m: usize = 100;
    let reds: Vec<usize> = vec![
        195, 144, 903, 63, 137, 513, 44, 626, 75, 473,
        876, 421, 568, 519, 755, 840, 374, 368, 570, 872,
        363, 650, 155, 265, 64, 26, 426, 391, 15, 421,
        373, 984, 564, 54, 823, 477, 565, 866, 879, 638,
        117, 755, 835, 683, 52, 369, 302, 424, 513, 870,
        75, 874, 299, 228, 140, 361, 30, 342, 750, 819,
        761, 123, 804, 325, 952, 405, 578, 517, 49, 457,
        932, 941, 988, 767, 624, 41, 912, 702, 241, 426,
        351, 92, 300, 648, 318, 216, 785, 347, 556, 535,
        166, 318, 434, 746, 419, 386, 928, 996, 680, 975,
    ];
    let blues: Vec<usize> = vec![
        231, 390, 916, 220, 933, 319, 37, 846, 797, 54,
        272, 924, 145, 348, 350, 239, 563, 135, 362, 119,
        446, 305, 213, 879, 51, 631, 43, 755, 405, 499,
        509, 412, 887, 203, 408, 821, 298, 443, 445, 96,
        274, 715, 796, 417, 839, 147, 654, 402, 280, 17,
        298, 725, 98, 287, 382, 923, 694, 201, 679, 99,
        699, 188, 288, 364, 389, 694, 185, 464, 138, 406,
        558, 188, 897, 354, 603, 737, 277, 35, 139, 556,
        826, 213, 59, 922, 499, 217, 846, 193, 416, 525,
        69, 115, 489, 355, 256, 654, 49, 439, 118, 961,
    ];
    let input: Vec<(usize, usize, usize, usize, usize)> = create_edges(n, m, reds, blues);
    let mut g: q16::Graph = q16::Graph::new(n + m + 2);
    g.add_all_edge(
        input.iter()
        .map(|x| (x.0, x.1, x.2 as u32))
        .collect()
    );
    println!("{}", q16::fordfulkerson(&mut g, 0, n + m + 1));
}

fn q16_5() {
    println!("question 16-5");
    println!("imperfect");
    let r: usize = 3;
    let c: usize = 3;
    let d: &str = "...
    ...
    ...";
    let data: Vec<Vec<char>> = word2vec(r, c, d);
    show_field(&data);
    let input: Vec<(usize, usize, u32)> = odds_evens(&data);
    let mut g: q16::Graph = q16::Graph::new(r * c + 2);
    g.add_all_edge(input);
    println!("{}", q16::fordfulkerson(&mut g, 0, r * c + 1));

    let r: usize = 2;
    let c: usize = 2;
    let d: &str = "**
    **";
    let data: Vec<Vec<char>> = word2vec(r, c, d);
    show_field(&data);
    let input: Vec<(usize, usize, u32)> = odds_evens(&data);
    let mut g: q16::Graph = q16::Graph::new(r * c + 2);
    g.add_all_edge(input);
    println!("{}", q16::fordfulkerson(&mut g, 0, r * c + 1));

    let r: usize = 1;
    let c: usize = 1;
    let d: &str = ".";
    let data: Vec<Vec<char>> = word2vec(r, c, d);
    show_field(&data);
    let input: Vec<(usize, usize, u32)> = odds_evens(&data);
    let mut g: q16::Graph = q16::Graph::new(r * c + 2);
    g.add_all_edge(input);
    println!("{}", q16::fordfulkerson(&mut g, 0, r * c + 1));

    let r: usize = 3;
    let c: usize = 4;
    let d: &str = "*..*
    ..**
    *...";
    let data: Vec<Vec<char>> = word2vec(r, c, d);
    show_field(&data);
    let input: Vec<(usize, usize, u32)> = odds_evens(&data);
    let mut g: q16::Graph = q16::Graph::new(r * c + 2);
    g.add_all_edge(input);
    println!("{}", q16::fordfulkerson(&mut g, 0, r * c + 1));
}

fn q16_6() {
    println!("question 16-6");
    let n: usize = 6;
    let a: Vec<isize> = vec![1, 2, -6, 4, 5, 3];
    let mut ff = FordFulkerson::new(n + 2);
    let mut sum: isize = 0;
    (sum, ff) = run(n, &a, &mut ff);
    let ans: isize = sum - ff.max_flow(0, n + 1);
    println!("{}", ans);

    let n: usize = 6;
    let a: Vec<isize> = vec![100, -100, -100, -100, 100, -100];
    let mut ff = FordFulkerson::new(n + 2);
    let mut sum: isize = 0;
    (sum, ff) = run(n, &a, &mut ff);
    let ans: isize = sum - ff.max_flow(0, n + 1);
    println!("{}", ans);
}

fn run(n: usize, a: &Vec<isize>, ff: &mut FordFulkerson) -> (isize, FordFulkerson) {
    let mut sum: isize = 0;
    for i in 1..=n {
        if a[i - 1] > 0 {
            sum += a[i - 1];
            ff.add_edge(i, n + 1, a[i - 1]);
        } else {
            ff.add_edge(0, i, -a[i - 1]);
        }
        for j in 2..=(n / i) {
            let baisu: usize = i * j;
            if a[baisu - 1] > 0 {
                ff.add_edge(i, baisu, a[baisu - 1]);
            }
        }
    }
    (sum, ff.clone())
}
