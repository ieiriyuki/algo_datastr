fn main() {
    println!("example 6-1");

    println!("question 6-1");
    let a = vec![12, 43, 7, 15, 9];
    println!("{}", q06_1(a, 9));
}

/*
fn c06_1(a: Vec<int>) {
    let left: usize = 0;
    let right: usize = a.len();

    while right >= left {
        let mid =
    }
}
*/

fn q06_1 (a: Vec<i32>, key: i32) -> usize {
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
