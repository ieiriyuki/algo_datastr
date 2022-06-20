use rand::prelude::*;
use std::collections::{BinaryHeap, HashMap};

fn main() {
    do_training();

    println!("question 12-1");
    let mut v = vec![8, 7, 6, 5, 4, 3, 2, 1];
    q12_1(v);

    println!("question 12-2");
    // Aでソートして小さい順にMになるまで買う

    println!("question 12-3");
    // それまでの順番とk番目の値を保持して、追加されるたびに都度比較する
    let v = vec![8, 7, 6, 5, 4, 3, 2, 1];
    q12_3(&v, 4);

    // question 12-4
    // 証明が面倒なので略

    println!("question 12-5");
    // 順に要素を追加していく, その都度 k 番目に小さい値を保持・比較する <- それではうまくいかない
    let v = vec![8, 7, 6, 5, 4, 3, 2, 1];
    println!("{}", q12_5(&v, 4));

    // question 12-6
    // ムズイ
}

fn do_training() {
    println!("merge sort");
    let mut v = vec![8, 7, 6, 5, 4, 3, 2, 1];
    println!("{:?}", merge_sort(v, 0, 8));
    let mut v = vec![13, 8, 3, 3, 2];
    println!("{:?}", merge_sort(v, 0, 5));

    println!("quick sort");
    let mut v = vec![8, 7, 6, 5, 4, 3, 2, 1];
    println!("{:?}", quick_sort(v, 0, 8));

    println!("heap sort");
    let mut v = vec![8, 7, 6, 5, 4, 3, 2, 1];
    println!("{:?}", heap_sort(v));
    let mut v = vec![8, 7, 6, 5, 4, 3, 2, 1].into_iter().rev().collect();
    println!("{:?}", heap_sort(v));

    println!("bucket sort");
    let mut v = vec![8, 7, 6, 5, 4, 3, 2, 1];
    println!("{:?}", bucket_sort(v));
    let mut v = vec![8, 7, 6, 5, 4, 3, 2, 1].into_iter().rev().collect();
    println!("{:?}", bucket_sort(v));
}

fn merge_sort(mut v: Vec<i32>, left: usize, right: usize) -> Vec<i32>{
    if right - left == 1 {
        return v
    }

    let mid = (right + left) / 2;
    v = merge_sort(v, left, mid);
    v = merge_sort(v, mid, right);

    let mut buf = Vec::<i32>::new();
    for i in left..mid {
        buf.push(v[i]);
    }
    for i in (mid..=(right - 1)).rev() {
        buf.push(v[i]);
    }

    let mut index_left = 0 as usize;
    let mut index_right = buf.len() - 1 as usize;
    for i in left..right {
        if buf[index_left] <= buf[index_right] {
            v[i] = buf[index_left];
            index_left += 1;
        } else {
            v[i] = buf[index_right];
            index_right -= 1;
        }
    }
    return v
}

fn quick_sort(mut v: Vec<i32>, left: usize, right: usize) -> Vec<i32> {
    if right - left <= 1 {
        return v
    }

    let mut rng = thread_rng();
    let pivot_index = rng.gen_range(left..right);
    let pivot = v[pivot_index];
    v.swap(pivot_index, right - 1);

    let mut i = left;
    for j in left..right {
        if v[j] < pivot {
            v.swap(i, j);
            i += 1;
        }
    }
    v.swap(i, right - 1);

    v = quick_sort(v, left, i);
    v = quick_sort(v, i + 1, right);

    return v
}

fn heapify(mut v: Vec<i32>, i: usize, n: usize) -> Vec<i32> {
    let mut left_child = i * 2 + 1;
    if left_child >= n {
        return v
    }

    if left_child + 1 < n && v[left_child + 1] > v[left_child] {
        left_child += 1;
    }
    if v[left_child] <= v[i] {
        return v
    }

    v.swap(i, left_child);
    v = heapify(v, left_child, n);

    return v
}

fn heap_sort(mut v: Vec<i32>) -> Vec<i32> {
    let n = v.len();

    for i in (0..(n / 2)).rev() {
        v = heapify(v, i, n);
    }

    for i in (1..n).rev() {
        v.swap(0, i);
        v = heapify(v, 0, i);
    }

    return v
}

fn bucket_sort(mut v: Vec<i32>) -> Vec<i32> {
    let MAX = 100_000;
    let mut num: Vec<i32> = vec![0; MAX];
    for i in 0..v.len() {
        num[v[i] as usize] += 1;
    }

    let mut sum: Vec<i32> = vec![0; MAX];
    for i in 1..MAX {
        let j = i as usize;
        sum[j] = sum[j - 1] + num[j];
    }

    let mut v2: Vec<i32> = vec![0; v.len()];
    for i in (0..v.len()).rev() {
        v2[(sum[v[i] as usize] - 1) as usize] = v[i];
    }

    return v2
}

fn q12_1(mut v: Vec<i32>) {
    let temp_v = v.clone();
    let n = temp_v.len();
    let sorted_v = merge_sort(v, 0, n);
    let mut order_of_x = HashMap::<i32, usize>::new();
    for i in 0..n {
        order_of_x.insert(sorted_v[i], i);
    }
    for v in temp_v.into_iter() {
        println!("{}: {}", v, order_of_x[&v]);
    }
}

fn q12_3(v: &Vec<i32>, k: usize) {
    let mut lowers = BinaryHeap::new();
    let mut highers = BinaryHeap::new();
    let mut counter = 0;
    for i in v.into_iter() {
        if counter < k {
            lowers.push(i);
        } else {
            let mut temp: i32 = 0;
            if let Some(mx) = lowers.pop() {
                temp = *mx;
            }
            if temp < *i {
                highers.push(*i);
            } else {
                highers.push(temp);
                lowers.push(&*i);
            }
        }
        counter += 1;
    }
    let result = lowers.pop().unwrap();
    println!("{}", result);
}


fn q12_5(v: &Vec<i32>, k: usize) -> i32 {
    let mut counter: usize = 0;
    let mut kth_smallest_value: i32 = -1 << 30;
    let mut prev_smallest_value: i32 = -1 << 30;
    for i in v.into_iter() {
        counter += 1;
        if counter < k {
            if kth_smallest_value <= *i {
                prev_smallest_value = kth_smallest_value;
                kth_smallest_value = *i;
            }
        } else {
            if *i <= kth_smallest_value {
                kth_smallest_value = *i;
            }
        }
    }
    println!("{}", kth_smallest_value);
    return 1
}
