use std::cmp::min;
use std::collections::{HashSet, HashMap};

fn main() {
    // q08-1
    // 先頭から順に見ていくので, O(N) では

    // q08-2
    // 1, 2, ... N 回アクセスするので
    // N(N + 1)/2 回になるはず
    // O(N^2) では

    // q08-3
    // リストの最初にサイズの値を保持する
    // 他の要素にはそのサイズへの参照を持つ

    // q08-4
    // リストの最初から探索する
    // ノードvにたどり着いたら, v を削除する
    // 前後のノードを連結させる

    // q08-5
    let a = vec![1, 2, 3, 4, 5, 6];
    let b = vec![4, 5, 6, 7];
    println!("{}", q08_5(a, b));

    // q08-6
    let a = vec![1, 2, 3, 4, 5, 6, 1, 3, 3];
    let b = vec![4, 5, 6, 7, 3, 3];
    println!("{}", q08_6(a, b));

    // q08-7
    let a = vec![1, 2, 3, 4];
    let b = vec![5, 6, 7, 8];
    println!("{}", q08_7(11, a, b));
}

fn q08_5(a_: Vec<i32>, b_: Vec<i32>) -> i32 {
    let a: HashSet<i32> = a_.into_iter().collect();
    let b: HashSet<i32> = b_.into_iter().collect();
    let mut n_common = 0;
    for i in &a {
        if !b.contains(i) {
            n_common += 1;
        }
    }
    return n_common
}

fn q08_6(a_: Vec<i32>, b_: Vec<i32>) -> i32 {
    // https://doc.rust-jp.rs/book-ja/ch08-03-hash-maps.html
    fn make_kvs(x: Vec<i32>, mut y: HashMap<i32, i32>) -> HashMap<i32, i32> {
        for i in x.into_iter() {
            let count = y.entry(i).or_insert(0);
            *count += 1;
        }
        // https://doc.rust-jp.rs/book-ja/ch04-01-what-is-ownership.html
        return y
    }

    // O(N)くらいの計算量
    let mut a: HashMap<i32, i32> = HashMap::new();
    a = make_kvs(a_, a);
    // O(M)くらいの計算量
    let mut b = HashMap::new();
    b = make_kvs(b_, b);

    // O(N)くらいの計算量
    let key_a = a.keys().collect::<HashSet<_>>();
    // O(M)くらいの計算量
    let key_b = b.keys().collect::<HashSet<_>>();
    // O(N)くらいの計算量
    let key_common: HashSet<_> = key_a.intersection(&key_b).collect();
    // println!("{:?}", key_common);

    // O(N)くらいの計算量
    let mut count = 0;
    for i in key_common {
        let x = a.get(i).unwrap();
        let y = b.get(i).unwrap();
        count += min(x, y);
    }

    return count
}

fn q08_7(k: i32, a_: Vec<i32>, b_: Vec<i32>) -> bool {
    if a_.len() != b_.len() {
        panic!("lengths are different!!!");
    }
    // O(N)
    let a: HashSet<_> = a_.into_iter().map(|x| k - x).collect();
    // O(N)
    let b: HashSet<_> = b_.into_iter().collect();
    // O(N)?
    let c = a.intersection(&b).collect::<HashSet<_>>().len();

    if c > 0 {
        return true
    } else {
        return false
    }
}
