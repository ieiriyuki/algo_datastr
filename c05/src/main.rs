fn main() {
    println!("Hello, world!");

    let ans: i32 = q05_1(2, &[10, 20, 30], &[40, 50, 60], &[70, 1000, 90]);
    println!("{}", ans);

    let ans: i32 = q05_1(1, &[100], &[10], &[1]);
    println!("{}", ans);

    let ans: i32 = q05_1(
        7,
        &[6, 8, 2, 7, 4, 2, 7],
        &[7, 8, 5, 8, 6, 3, 5],
        &[8, 3, 2, 6, 8, 4, 1],
    );
    println!("{}", ans);
}

fn q05_1(N: usize, a: &[i32], b: &[i32], c: &[i32]) -> i32{
    let mut gain: Vec<Vec<i32>> = vec![vec![0; 3]; N];
    for i in 0..N {
        for j in 0..3 {
            gain_i[i][j] = temp;
            gain_i[i][1] = activity;
        }
        else {
            let mut temp = gain_i[i -1][0] + a[i];
            let mut activity = 0;
            if (temp < gain_i[i -1][0] + b[i])
                && (gain_i[i -1][1] != 1) {
                temp = gain_i[i -1][0] + b[i];
                activity = 1;
            }
            if (temp < gain_i[i -1][0] + c[i])
                && (gain_i[i -1][1] != 2) {
                temp = gain_i[i -1][0] + c[i];
                activity = 2;
            }
            gain_i[i][0] = temp;
            gain_i[i][1] = activity;
        }
    }
    println!("{:?}", gain_i);
    gain_i[N-1][0]
}
