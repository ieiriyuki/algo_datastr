fn main() {
    println!("Hello, world!");

    println!("question 6-1");
    let a = vec![12, 43, 7, 15, 9];
    println!("{:?}", &q06_1(&a));

    println!("question 6-2");
    let a = vec![12, 43, 7, 15, 9];
    let b = vec![11, 12, 13, 14, 15];
    let c = vec![11, 13, 17, 23, 29];
    println!("{:?}", q06_2(&a, &b, &c));
    // let c = vec![11, 13, 17, 23, 29, 99];
    // println!("{:?}", &q06_2(&a, &b, &c));  // panic should happen
    let a = vec![12, 43, 7, 15, 9];
    let b = vec![0, 1, 2, 3, 4];
    assert_eq!(0, q06_2(&a, &b, &c));
    let b = vec![10, 11, 12, 13, 14];
    let c = vec![0, 1, 2, 3, 4];
    assert_eq!(0, q06_2(&a, &b, &c));
}

fn q06_1 (a: &Vec<i32>) -> Vec<usize> {
    let mut a_ = a.clone();
    a_.sort();
    let mut order: Vec<usize> = vec![0; a.len()];

    for i in 0..a_.len() {
        let mut left: usize = 0;
        let mut right: usize = a_.len() - 1;
        let mut mid: usize;
        while left <= right {
            mid = (left + right) / 2;
            if a[i] == a_[mid] {
                order[i] = mid;
                break;
            } else if a[i] < a_[mid] {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
    }
    return order
}

fn q06_2(a: &Vec<i32>, b: &Vec<i32>, c: &Vec<i32>) -> i32 {
    if a.len() != b.len() || a.len() != c.len() {
        panic!("lengths of vectors are different. {} != {} != {}", a.len(), b.len(), c.len());
    }
    let mut a_ = a.clone();
    a_.sort();
    let mut b_ = b.clone();
    b_.sort();
    let mut c_ = c.clone();
    c_.sort();

    if a_[0] > b_[b_.len() - 1] {
        return 0
    }
    if b_[0] > c_[c_.len() - 1] {
        return 0
    }

    // b[i] より大きい c[j] の数
    let mut n_c_over_b: Vec<i32> = vec![0; b_.len()];
    for i in 0..b_.len() {
        let idx = lower_bound_index(b[i], &c_);
        if idx == -1 {
            n_c_over_b[i] = 0;
        } else {
            n_c_over_b[i] = c_.len() as i32 - idx;
        }
    }
    println!("{:?}", n_c_over_b);

    // a[i] より大きい b[j] があるとき, b[j] より大きい c[k] の数を合計
    let mut n_b_over_a: Vec<i32> = vec![0; a_.len()];
    for i in 0..a_.len() {
        let idx = lower_bound_index(a_[i], &b_);
        if idx == -1 {
            n_b_over_a[i] = 0;
        } else {
            let idx_ = idx as usize;
            let sum: i32 = n_c_over_b[idx_..].into_iter().sum();
            n_b_over_a[i] = sum;
        }
    }
    println!("{:?}", n_b_over_a);

    return 1
}

fn lower_bound_index(key: i32, x: &Vec<i32>) -> i32 {
    // x is supporsed to be sorted

    let mut left = 0;
    let mut right = x.len() - 1;
    if x[right] <= key {
        return -1
    }
    while left < right {
        let mid = (left + right) / 2;
        if key < x[mid] {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    return right as i32
}
