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
    let r_x = vec![3, 4, 7, 9];
    let r_y = r_x.clone();
    let b_x = r_x.clone();
    let b_y = vec![3, 4, 7, 9, 0];
    println!("{:?}", q07_2(&r_x, &r_y, &b_x, &b_y).expect_err(""));
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
    return Ok(1)
}

