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

    println!("question 6-1");
    println!("{}", q06_1(9, &a));
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

fn q06_1 (key: i32, a: &Vec<i32>) -> usize {
    let mut b: Vec<i32> = a.clone();
    b.sort();
    let mut left: usize = 0;
    let mut right = a.len();
    let mut mid: usize = 0;

    while right >= left {
        mid = (left + right) / 2;
        if a[mid] == key {
            return mid
        } else if a[mid] > key {
            right = mid -1;
        } else if a[mid] < key {
            left = mid + 1;
        }
    }
    return mid
}
