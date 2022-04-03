fn main() {
    println!("Hello, world!");

    println!("question 6-1");
    let a = vec![12, 43, 7, 15, 9];
    println!("{:?}", &q06_1(&a));

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
