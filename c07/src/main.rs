use std::iter::zip;

fn main() {
    println!("test lower bound");
    let a = vec![1, 3, 5, 7, 9];
    assert_eq!(3, lower_bound(6, &a).unwrap());
    assert_eq!(4, lower_bound(7, &a).unwrap());
    assert_eq!(0, lower_bound(0, &a).unwrap());
    assert_eq!(-1, lower_bound(9, &a).expect_err(""));
    let a = vec![1, 3, 5, 7, 9, 11];
    assert_eq!(2, lower_bound(4, &a).unwrap());
    assert_eq!(5, lower_bound(9, &a).unwrap());
    assert_eq!(0, lower_bound(0, &a).unwrap());
    assert_eq!(-1, lower_bound(11, &a).expect_err(""));

    println!("question 7-1");
    let a = vec![1, 2, 3];
    let b = vec![2, 3, 4];
    println!("{}", q07_1(&a, &b));
    let a = vec![3, 4, 7, 9];
    let b = vec![2, 5, 7, 8];
    println!("{}", q07_1(&a, &b));

    println!("question 7-2");
    let r_x = vec![7, 4, 3, 9];
    let r_y = r_x.clone();
    let b_x = r_x.clone();
    let b_y = vec![3, 4, 7, 9, 0];
    println!("{:?}", q07_2(&r_x, &r_y, &b_x, &b_y).expect_err(""));
    let b_x = vec![8, 5, 6, 2];
    let b_y = b_x.clone();
    println!("{:?}", q07_2(&r_x, &r_y, &b_x, &b_y).unwrap());

    println!("question 7-3");
    let d = vec![3, 4, 1, 4];
    let t = vec![3, 6, 9, 12];
    println!("{}", q07_3(&d, &t));
    let d = vec![3, 4, 4, 1];
    let t = vec![3, 12, 9, 6];
    println!("{}", q07_3(&d, &t));
}

fn q07_1(_a: &Vec<i32>, _b: &Vec<i32>) -> i32 {
    let mut a = _a.clone();
    let mut b = _b.clone();
    a.reverse();
    b.sort();

    let mut counter = 0;
    for i in 0..a.len() {
        let idx: i32 = match lower_bound(a[i], &b) {
            Ok(idx) => idx as i32,
            Err(mns) => mns,
        };
        if idx < 0 {
            continue
        } else {
            counter += 1;
            b.remove(idx as usize);
        }
    }
    return counter
}

fn lower_bound(key: i32, _a: &Vec<i32>) -> Result<usize, i32> {
    // key < a[i] を満たす最小の a[i] について i を返す
    let mut a = _a.clone();
    a.sort();
    if a[a.len() - 1] <= key {
        return Err(-1)
    }

    let mut left = 0;
    let mut right = a.len() - 1;
    while left < right {
        let mid = (left + right) / 2;
        if key < a[mid] {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    return Ok(left)
}

// skip
fn q07_2(
    _r_x: &Vec<i32>,
    _r_y: &Vec<i32>,
    _b_x: &Vec<i32>,
    _b_y: &Vec<i32>,
) -> Result<i32, ()> {
    if _r_x.len() != _r_y.len() || _r_x.len() != _b_x.len() || _r_x.len() != _b_y.len() {
        return Err(())
    }
    // xかつyでソート
    let mut reds: Vec<_> = zip(_r_x, _r_y).map(|x| (*x.0, *x.1)).collect();
    reds.sort_by(|x, y| x.0.cmp(&y.0).then(x.1.cmp(&y.1)));
    let mut blues: Vec<_> = zip(_b_x, _b_y).map(|x| (*x.0, *x.1)).collect();
    blues.sort_by(|x, y| x.0.cmp(&y.0).then(x.1.cmp(&y.1)));

    // println!("{:?}", blues);
    /* debug
    for x in reds {
        print_type(&x);
        print_type(&x.0);
    }
    */
    let mut counter = 0;
    let mut j: usize = 0;
    for i in 0..reds.len() {
        while j < blues.len() {
            if reds[i].0 < blues[j].0 && reds[i].1 < blues[j].1 {
                counter += 1;
                j += 1;
                break
            }
            j += 1;
        }
    }
    return Ok(counter);
}

#[allow(dead_code)]
fn print_type<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn q07_3(_d: &Vec<i32>, _t: &Vec<i32>) -> bool {
    if _d.len() != _t.len() {
        panic!("lengths are different!")
    }
    // tでソートしないとダメ
    let mut d = _d.clone();
    let mut t = _t.clone();

    let mut n_try: usize = 0;
    let n_limit = d.len();
    let mut t_cur: i32 = 0;
    while n_try < n_limit {
        n_try += 1;

        let n_remain = d.len();
        let mut i_to_remove: Vec<usize> = vec![];
        for i in 0..n_remain {
            println!("{} {} {} {} {}", n_try, i, t_cur, d[i], t[i]);
            if t_cur + d[i] <= t[i] {
                t_cur += d[i];
                i_to_remove.push(i);
            }
        }
        i_to_remove.reverse();
        println!("{:?}", i_to_remove);
        for i in 0..i_to_remove.len() {
            d.remove(i_to_remove[i]);
            t.remove(i_to_remove[i]);
        }
    }
    if d.len() > 0 {
        return false
    } else {
        return true
    }
}
