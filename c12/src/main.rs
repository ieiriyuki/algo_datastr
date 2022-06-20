use rand::prelude::*;

fn main() {
    do_training();
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
