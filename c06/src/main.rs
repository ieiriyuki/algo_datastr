use std::io;

fn main() {
    let a = vec![12, 43, 7, 15, 9];
    println!("example 6-1");
    println!("{}", c06_1(10, &a));
    println!("{}", c06_1(15, &a));

    println!("example 6-3");
    c06_3(10, 11);

    println!("example 6-4");
    let b = vec![1, 2, 3, 4, 5];
    println!("{}", c06_4(40, &a, &b));
    println!("{}", c06_4(8, &a, &b));
    println!("{}", c06_4(20, &a, &b));

    println!("test");
    println!("{}", lower_bound(6, &vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]));
    println!("{}", lower_bound(11, &vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]));
    println!("{}", lower_bound(0, &vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]));
    println!("{}", lower_bound(-9, &vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]));

    println!("example 6-5");
    let a: Vec<i32> = vec![1, 2, 3, 4, 5];
    let b: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!("{}", c06_5(&a, &b));
    let a: Vec<i32> = vec![1; 5];
    let b: Vec<i32> = vec![1; 5];
    println!("{}", c06_5(&a, &b));

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

    println!("test upper_bound");
    let c = vec![1, 3, 3, 5, 7];
    match upper_bound(0, &c) {
        Ok(idx) => println!("success: {}", idx),
        Err(num) => println!("fail: {}", num)
    };
    match upper_bound(4, &c) {
        Ok(idx) => println!("success: {}", idx),
        Err(num) => println!("fail: {}", num)
    };
    match upper_bound(2, &c) {
        Ok(idx) => println!("success: {}", idx),
        Err(num) => println!("fail: {}", num)
    };
    match upper_bound(8, &c) {
        Ok(idx) => println!("success: {}", idx),
        Err(num) => println!("fail: {}", num)
    };
    let c = vec![6, 8, 8, 10];
    match upper_bound(7, &c) {
        Ok(idx) => println!("success: {}", idx),
        Err(num) => println!("fail: {}", num)
    };
    match upper_bound(5, &c) {
        Ok(idx) => println!("success: {}", idx),
        Err(num) => println!("fail: {}", num)
    };
    match upper_bound(7, &c) {
        Ok(idx) => println!("success: {}", idx),
        Err(num) => println!("fail: {}", num)
    };
    println!("even length");
    let c = vec![2, 3, 3, 4];
    match upper_bound(4, &c) {
        Ok(idx) => println!("success: {}", idx),
        Err(num) => println!("fail: {}", num)
    };
    match upper_bound(3, &c) {
        Ok(idx) => println!("success: {}", idx),
        Err(num) => println!("fail: {}", num)
    };
    match upper_bound(2, &c) {
        Ok(idx) => println!("success: {}", idx),
        Err(num) => println!("fail: {}", num)
    };

    println!("question 6-3");
    let a = vec![1, 2];
    assert_eq!(-1, q06_3(0, &a));
    let a = vec![3, 2];
    assert_eq!(-1, q06_3(3, &a));
    let a = vec![1, 0];
    println!("{}", q06_3(5, &a));
    let a = vec![2, 1];
    println!("{}", q06_3(6, &a));
    let a = vec![3, 5];
    println!("{}", q06_3(13, &a));
    println!("{}", q06_3(15, &a));

    println!("question 6-4");
    println!("{}", q06_4(1, &vec![0, 1, 2, 3]).expect_err("will err"));
    println!("{}", q06_4(2, &vec![0]).expect_err("short vector"));
    println!("{}", q06_4(2, &vec![1, 3]).unwrap());
    let a = vec![0, 10, 30, 55, 70, 85, 100];
    println!("{}", q06_4(4, &a).unwrap());

    println!("question 6-5");
    let a = vec![1, 2, 3, 31, 32, 33];
    let b = vec![5, 7, 11, 13, 101, 199, 0];
    println!("will fail: {}", q06_5(1, &a, &b).expect_err(""));
    let a = vec![1, 5];
    let b = vec![2, 3];
    println!("{}", q06_5(1, &a, &b).unwrap());
    println!("{}", q06_5(2, &a, &b).unwrap());
    println!("{}", q06_5(3, &a, &b).unwrap());
    let a = vec![1, 7, 11];
    let b = vec![2, 3, 13];
    println!("{}", q06_5(1, &a, &b).unwrap());

    println!("question 6-6");
    println!("{}", q06_6(1, 1, 1));
    println!("{}", q06_6(53, 82, 49));
    println!("{}", q06_6(99, 1, 1));
    println!("{}", q06_6(110, 10, 1));

    println!("question 6-7");
    let a = vec![1, 2, 3];
    println!("{}", q06_7(&a));
}

fn c06_1(key: i32, a: &Vec<i32>) -> i32 {
    let mut left: usize = 0;
    let mut right: usize = a.len() - 1;
    let mut mid: usize;

    while right >= left {
        mid = left + (right - left) / 2;
        if a[mid] == key {
            return mid as i32
        } else if key < a[mid] {
            right = mid - 1;
        } else if a[mid] < key {
            left = mid + 1;
        }
    }
    return -1
}

fn c06_3(left: usize, right: usize) {
    let mut left_: usize = left;
    let mut right_: usize = right;
    let mut mid: usize = left;

    while right_ - left_ > 1 {
        mid = (left_ + right_) / 2;
        println!("is older than {}? (yes/no)", mid);
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).expect("failed to read input");

        match buffer.trim() {
            "yes" => left_ = mid + 1,
            "no" => right_ = mid - 1,
            _ => println!("answer yes / no"),
        }
    }
    println!("you are {} years old", mid)
}

fn c06_4(k: i32, a: &Vec<i32>, b: &Vec<i32>) -> i32 {
    let mut a_ = a.clone();
    a_.sort();
    let mut b_ = b.clone();
    b_.sort();
    let mut minimum: i32 = 1 << 30;

    for i in 0..a_.len() {
        let temp = k - a_[i]; // k - a_[i] <= b[j]

        let b_min = lower_bound(temp, &b_);
        if b_min == -1 {
            continue
        }
        if a_[i] + b_min <= minimum { minimum = a_[i] + b_min; }
    }
    return minimum
}

fn lower_bound(key: i32, b: &Vec<i32>) -> i32 {
    let mut b_: Vec<i32> = b.clone();
    b_.sort();
    if b_[b_.len() - 1] < key { return -1 }

    let mut left: usize = 0;
    let mut right: usize = b.len() - 1;
    let mut mid: usize;

    while left < right {
        mid = (left + right) / 2;
        if b[mid] == key {
            return b_[mid]
        } else if key < b[mid] {
            right = mid;
        } else if b[mid] < key {
            left = mid + 1;
        }
    }
    return b_[right]
}

fn c06_5 (h: &Vec<i32>, s: &Vec<i32>) -> usize {
    if h.len() != s.len() { panic!("lengths are different! {} != {}", h.len(), s.len()) }
    let inf_: usize = 1 << 30;
    let mut left: usize = 0;
    let mut right = inf_;

    while right - left > 1 {
        let mid = (left + right) / 2;
        let mut is_ok = true;
        let mut t: Vec<i32> = vec![0; h.len()];

        for i in 0..h.len() {
            if h[i] > mid as i32 {
                is_ok = false;
            } else {
                t[i] = (mid as i32 - h[i]) / s[i];
            }
        }
        t.sort();
        for i in 0..h.len() {
            if t[i] < i as i32 { is_ok = false; }
        }
        if is_ok {
            right = mid;
        } else {
            left = mid;
        }
    }
    return right
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

fn q06_3(m: i32, a: &Vec<i32>) -> i32 {
    let mut a_ = a.clone();
    a_.sort();
    if m < a_[0] {
        println!("None of a[i] is less than m. {} < {}", m, a_[0]);
        return -1
    }
    let mut sums = vec![0; a_.len() * a_.len()];

    for i in 0..a_.len() {
        for j in 0..a_.len() {
            sums[i * a_.len() + j] = a_[i] + a_[j];
        }
    }
    sums.sort();
    if m < sums[0] {
        println!("No combination of a[i] is less than {}. minimum = {}", m, sums[0]);
        return -1
    }
    println!("{:?}", sums);

    let mut maximum: i32 = 0;
    for i in 0..a_.len() {
        for j in 0..a_.len() {
            let sum_of_two_1 = sums[i * a_.len() + j];
            let criterion = m - sum_of_two_1;
            let sum_of_two_2 = match upper_bound(criterion, &sums) {
                Ok(idx) => sums[idx],
                Err(_) => 0
            };
            if maximum < sum_of_two_1 + sum_of_two_2 {
                maximum = sum_of_two_1 + sum_of_two_2;
            }
        }
    }
    return maximum
}

fn upper_bound(key: i32, x: &Vec<i32>) -> Result<usize, i32> {
    let mut left: usize = 0;
    let mut right = x.len() - 1;
    if key < x[left] {
        return Err(-1)
    }
    while left <= right {
        let mid = (left + right) / 2;
        if x[mid] == key {
            left = mid + 1;
            right = mid;
        } else if key < x[mid] {
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }
    return Ok(right)
}

fn q06_4 (m: i32, a: &Vec<i32>) -> Result<i32, i32> {
    // 上のほう以外は回答の模写
    if m < 2 {
        println!("m should be >= 2");
        return Err(-1)
    }
    if a.len() < 2 {
        println!("vector length should be >= 2");
        return Err(-1)
    }
    if m == 2 {
        return Ok(a[a.len() - 1] - a[0])
    }
    let mut a_ = a.clone();
    a_.sort();

    let mut left = 0;
    let mut right = 1 << 30;
    while left + 1 < right {
        let mid = (left + right) / 2;
        let mut count = 1;
        let mut prev: usize = 0;
        for i in 1..a_.len() {
            if mid <= (a_[i] - a[prev]) {
                count += 1;
                prev = i;
            }
        }
        if count >= m {
            left = mid;
        } else {
            right = mid;
        }
    }
    return Ok(left)
}

fn q06_5(k: usize, a_: &Vec<i32>, b_: &Vec<i32>) -> Result<usize, i32> {
    // Incomplete implementation, basic logic is correct

    if a_.len() != b_.len() {
        return Err(-1)
    }

    let mut a = a_.clone();
    a.sort();
    let mut b = b_.clone();
    b.sort();
    let n = a.len();

    let mut left: usize = 0;
    let mut right: usize = 1 << 30;
    while left + 1 < right {
        let mid = (left + right) / 2;
        let mut count = 0;

        for i in 0..n {

            let mut left_2: usize = 0;
            let mut right_2: usize = n - 1;
            while left_2 < right_2 {
                let mid_2 = (left_2 + right_2) / 2;
                if a[i] * b[mid_2] < (mid as i32) {
                    count += 1;
                    left_2 = mid_2 + 1;
                } else {
                    right_2 = mid_2;
                }
            }
            count += left_2;
        }

        if k <= count {
            left = mid;
        } else {
            right = mid;
        }
    }
    return Ok(left)
}

fn q06_6(a: i32, b: i32, c: i32) -> f64 {
    use std::f64::consts::PI;
    let calc = |a: i32, b: i32, c: i32, t: f64| {
        (a as f64) * t + (b as f64) * ((c as f64) * t * PI).sin()
    };

    let mut left: f64 = 0.;
    let mut right: f64 = 400.;
    let mut is_close = false;
    let eps = 1e-10;
    while !is_close {
        let mid: f64 = (left + right) / 2.;
        let val = calc(a, b, c, mid);
        if (val - 100.).abs() < eps {
            is_close = true;
            left = mid;
        } else if 100. < val {
            right = mid;
        } else {
            left = mid;
        }
    }
    return left
}

fn q06_7(a_: &Vec<i32>) -> i32 {
    let mut a = a_.clone();
    a.sort();

    let mut left = 0;
    let mut right = 1 << 30;
    while left + 1 < right {
        let mid = (left + right) / 2;

        if true {
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }
    return 1
}
